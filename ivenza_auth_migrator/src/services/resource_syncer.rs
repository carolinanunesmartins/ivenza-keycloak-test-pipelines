use super::IvenzaClient;
use super::KeycloakClient;
use crate::models::keycloak::ScopeResponse;
use std::error::Error;

pub struct ResourceSyncer;

impl ResourceSyncer {
    /// Determines which resources should be extracted from the ivenza permissions
    /// Validates which resources are already available within keycloak
    /// and inserts missing resources into keycloak.
    pub async fn sync() -> Result<(), Box<dyn Error>> {
        // Get all the scopes from ivenza
        let scopes = IvenzaClient::determine_scopes();

        // Get all permissions from ivenza.
        let permissions = IvenzaClient::get_permissions();

        // Determine which resource we're expecting to be present in keycloak.
        let ivenza_resources = IvenzaClient::determine_resources(&permissions, &scopes);

        // Create a new keycloak client.
        let mut keycloak_client = KeycloakClient::new();

        // Retrieve the known resources from keycloak.
        let keycloak_resouces = keycloak_client.get_resources().await?;

        // Get the known scopes from keycloak.
        let keycloak_scopes = keycloak_client.get_scopes().await?;

        // Determine which resource are missing in keycloak.
        let missing_resources = ivenza_resources.iter().filter(|&ir| {
            !keycloak_resouces
                .iter()
                .any(|kcr| kcr.name.to_lowercase().eq(&ir.0.to_lowercase()))
        });

        // Iterate over the missing resources.
        for missing_resource in missing_resources {
            // Filter the scopes we want for this resource, based on the attached scopes in the ivenza resource.
            let keycloak_scopes_for_resource = missing_resource
                .1
                .iter()
                .map(|ic| {
                    keycloak_scopes
                        .iter()
                        .find(|&scope| scope.name.to_lowercase().eq(&ic.to_lowercase()))
                })
                .filter(|kp| kp.is_some())
                .map(|kp| kp.unwrap())
                .collect::<Vec<&ScopeResponse>>();

            // If we have missing scopes in keycloak, we should notify the runner of it.
            if keycloak_scopes_for_resource.len() < missing_resource.1.len() {
                Self::print_missing_scopes_warning(&keycloak_scopes_for_resource, missing_resource);
                // continue with the next iteration
                continue;
            }

            println!(
                "Adding resource {} with {} scopes in keycloak",
                missing_resource.0,
                missing_resource.1.len()
            );

            // Insert the missing resouce into keycloak, and assign all possible scopes to it.
            keycloak_client
                .insert_resource(missing_resource.0, keycloak_scopes_for_resource)
                .await?;
        }

        Ok(())
    }

    /// prints our the found scopes in ivenza, and the found scopes in Keycloak.
    /// Use this for debugging and analysis purposes
    fn print_missing_scopes_warning(
        keycloak_scopes: &Vec<&ScopeResponse>,
        missing_resource: (&&str, &Vec<String>),
    ) {
        println!(
            "Warning, missing {} scope(s) for {}",
            missing_resource.1.len() - keycloak_scopes.len(),
            missing_resource.0
        );
        println!("Ivenza scopes: {:?}", missing_resource.1);
        for keycloak_scopes in keycloak_scopes {
            println!("Found : {}", keycloak_scopes.name)
        }
    }
}

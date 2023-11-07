use super::IvenzaClient;
use super::KeycloakClient;
use std::error::Error;

pub struct PolicySyncer;

impl PolicySyncer {
    /// Creates keycloak role based policies for every role defined in ivenza.
    pub async fn sync(ivenza_client: &IvenzaClient) -> Result<(), Box<dyn Error>> {
        // Get all the roles from ivena.
        let ivenza_roles = ivenza_client.get_roles().await;

        // construct the keycloak client
        let mut keycloak_client = KeycloakClient::new();

        // Get alll the known policies from keycloak.
        let keycloak_policies = keycloak_client.get_policies().await?;

        // get the current known roles from keycloak.
        let keycloak_roles = keycloak_client.get_roles().await?;

        // determine which roles in keycloak do not have a role-based policy assigned in keycloak.
        let keycloak_roles_without_policy = keycloak_roles
            .iter()
            .filter(|&kcr| {
                !keycloak_policies
                .iter()
                // Get all roles that are not assigned to a policy yet.
                .any(|kcp| kcp.config.get_roles().is_some() &&
                    kcp.config.get_roles()
                        .unwrap()
                        .iter()
                        .any(|kcpr| kcpr.id.eq(&kcr.id))
                ) &&
                // and are defined as a role in Ivenza, this filters our default realm roles from keycloak.
                ivenza_roles.iter().any(|ir| ir.name.to_lowercase().eq(&kcr.name.to_lowercase()))
            })
            .into_iter();

        // Insert missing policies in keycloak
        for keycloak_role_without_policy in keycloak_roles_without_policy {
            println!(
                "Inserting role based policy for role {} in Keycloak.",
                keycloak_role_without_policy.name
            );
            keycloak_client
                .insert_role_based_policy(&keycloak_role_without_policy)
                .await?
        }

        Ok(())
    }
}

use super::IvenzaClient;
use super::KeycloakClient;
use std::error::Error;

pub struct ScopeSyncer;

impl ScopeSyncer {
    /// Determines which scopes are expected to be in keycloak based on Ivenza configuration.
    /// If there are scopes missing from Ivenza, this function will insert them into keycloak.
    pub async fn sync(ivenza_client: &IvenzaClient) -> Result<(), Box<dyn Error>> {
        let ivenza_scopes = ivenza_client.determine_scopes().await;
        let mut keycloak_client = KeycloakClient::new();
        let keycloak_scopes = keycloak_client.get_scopes().await?;
        let missing_scopes = ivenza_scopes.iter().filter(|&is| {
            !keycloak_scopes
                .iter()
                .any(|ks| ks.name.to_lowercase().eq(&is.to_lowercase()))
        });

        // Insert the missing role in Keycloak.
        for missing_scope in missing_scopes {
            println!("Inserting scope {} into keycloak", missing_scope);
            keycloak_client.insert_scope(&missing_scope).await?;
        }
        Ok(())
    }
}

use super::IvenzaClient;
use super::KeycloakClient;
use std::error::Error;

pub struct RoleSyncer;

impl RoleSyncer {
    /// Synchronizes roles from Ivenza to keycloak.
    pub async fn sync() -> Result<(), Box<dyn Error>> {
        // Get all the known roles in ivenza.
        let ivenza_roles = IvenzaClient::get_roles();

        // Retrieve the known roles from Keycloak.
        let mut keycloak_client = KeycloakClient::new();
        let keycloak_roles = keycloak_client.get_roles().await?;

        // Check which roles are not available in keycloak, but are available in Ivenza
        let missing_roles = ivenza_roles.iter().filter(|&ir| {
            !keycloak_roles
                .iter()
                .any(|kr| kr.name.to_lowercase().eq(&ir.name.to_lowercase()))
        });

        // Insert the missing role in Keycloak.
        for missing_role in missing_roles {
            println!("Inserting role {} into keycloak", missing_role.name);
            keycloak_client
                .insert_role(&missing_role.name, &missing_role.display_name)
                .await?;
        }
        Ok(())
    }
}

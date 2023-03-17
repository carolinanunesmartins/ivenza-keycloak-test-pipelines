use super::IvenzaClient;
use super::KeycloakClient;
use std::error::Error;

use std::env;
pub struct UserSyncer;

const SKIP_INTERNAL_USERS_KEY: &str = "SKIP_INTERNAL_USERS";

impl UserSyncer {
    /// Synchronizes users from Ivenza to keycloak.
    pub async fn sync() -> Result<(), Box<dyn Error>> {
        let skip_internal_users = env::var(SKIP_INTERNAL_USERS_KEY)
            .unwrap_or_default()
            .eq("true");

        // Get all the known users in ivenza.
        let mut ivenza_users = IvenzaClient::get_users();
        if skip_internal_users {
            ivenza_users = ivenza_users
                .into_iter()
                .filter(|u| {
                    !u.email.contains("@uniconcreation.com") && !u.email.contains("@delihome.com")
                })
                .collect();
        }

        println!("Retrieved {} users", ivenza_users.len());
        // Retrieve the known users from Keycloak.
        let mut keycloak_client = KeycloakClient::new();
        let keycloak_users = keycloak_client.get_users().await?;
        let keycloak_roles = keycloak_client.get_roles().await?;

        // Check which users are not available in keycloak, but are available in Ivenza
        let missing_users = ivenza_users.iter().filter(|&ir| {
            !keycloak_users.iter().any(|kr| {
                kr.user_name
                    .to_lowercase()
                    .eq(&ir.login_name.to_lowercase())
            })
        });
        //
        // // Insert the missing user in Keycloak.
        for missing_user in missing_users {
            println!("Inserting user {} into keycloak", missing_user.login_name);
            match keycloak_roles
                .iter()
                .find(|r| missing_user.role.eq(&r.name))
            {
                Some(role) => {
                    keycloak_client.insert_user(&missing_user, &role).await?;
                }
                None => {
                    println!(
                        "Skipping user {} because the role {} cannot be found",
                        &missing_user.login_name, &missing_user.role
                    );
                }
            }
        }
        Ok(())
    }
}

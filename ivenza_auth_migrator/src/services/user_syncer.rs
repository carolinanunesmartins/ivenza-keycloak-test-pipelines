use crate::models::ivenza::User;

use super::IvenzaClient;
use super::KeycloakClient;
use std::error::Error;

use std::env;
pub struct UserSyncer;

const SKIP_INTERNAL_USERS_KEY: &str = "SKIP_INTERNAL_USERS";

impl UserSyncer {
    /// Synchronizes users from Ivenza to keycloak.
    pub async fn sync(ivenza_client: &IvenzaClient) -> Result<(), Box<dyn Error>> {
        let skip_internal_users = env::var(SKIP_INTERNAL_USERS_KEY)
            .unwrap_or_default()
            .eq("true");

        // Get all the known users in ivenza.
        let mut ivenza_users = ivenza_client.get_users().await;
        if skip_internal_users {
            ivenza_users = ivenza_users
                .into_iter()
                .filter(|u| {
                    !u.email
                        .clone()
                        .unwrap_or_default()
                        .contains("@uniconcreation.com")
                        && !u
                            .email
                            .clone()
                            .unwrap_or_default()
                            .contains("@delihome.com")
                })
                .collect();
        }

        println!("Found {} unique users in Ivenza", ivenza_users.len());
        // Retrieve the known users from Keycloak.
        let mut keycloak_client = KeycloakClient::new();
        let keycloak_users = keycloak_client.get_users().await?;
        println!("Found {} unique users in Keycloak", ivenza_users.len());
        let keycloak_roles = keycloak_client.get_roles().await?;

        // Check which users are not available in keycloak, but are available in Ivenza
        let missing_users = &ivenza_users
            .iter()
            .filter(|ir| {
                !keycloak_users.iter().any(|kr| {
                    kr.user_name
                        .to_lowercase()
                        .eq(&ir.login_name.to_lowercase())
                })
            })
            .collect::<Vec<&User>>();

        println!(
            "Found {} users present in both Ivenza and Keycloak",
            &ivenza_users.len() - &missing_users.len()
        );
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

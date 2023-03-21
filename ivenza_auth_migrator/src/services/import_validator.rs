use crate::models::ivenza::Permission;
use crate::services::ivenza_client::IvenzaClient;
use crate::services::keycloak_client::KeycloakClient;
use std::error::Error;

use super::ROOT_LEVEL_SCOPE;

pub struct ImportValidator;

impl ImportValidator {
    /// Validates that all permissions from Ivenza can be constructed from permissions in Keycloak
    pub async fn validate() -> Result<(), Box<dyn Error>> {
        // get all permissions from ivenza
        let source_permissions: Vec<Permission> = IvenzaClient::get_permissions();

        // construct the keycloak client
        let mut keycloak_client = KeycloakClient::new();

        // get all permissions from keycloak.
        let keycloak_permissions = keycloak_client.get_permissions().await?;

        // iterate over the keycloak permissions and flatten them to a list of Permissions like
        // modelled in the ivenza database
        let mut flat_permission_results: Vec<Permission> = vec![];
        for permission in &keycloak_permissions {
            // sanity check that associated_resources are set for a permission
            if let Some(resources) = &permission.associated_resources {
                // In our case, this is always one, so if resources are set, we also expect the first
                // to be available.
                if let Some(resource) = resources.first() {
                    // for each for policy
                    for role in permission.associated_role_policies.as_ref().unwrap() {
                        let role_name = role.get_role_name();
                        // root level permissions without additional scopes, can occur, but not always.
                        // Since we're constructing on resource.scope, we could also get matches by just the resource name.
                        // We don't want these if they don't exists in ivenza.
                        if source_permissions
                            .iter()
                            .any(|p| p.permission.eq(&resource.name))
                        {
                            flat_permission_results.push(Permission {
                                role: role_name.clone(),
                                permission: resource.name.to_string(),
                            });
                        }

                        // also create a permission for each scope, assigned to this resource for
                        // thie role.
                        if let Some(scopes) = &permission.associated_scopes {
                            for scope in scopes.iter().filter(|s| !s.name.eq(ROOT_LEVEL_SCOPE)) {
                                let kc_permission = Permission {
                                    role: role_name.clone(),
                                    permission: format!("{}.{}", resource.name, scope.name),
                                };
                                // unique push for the flattened permission list.
                                if !flat_permission_results.iter().any(|p| p.eq(&kc_permission)) {
                                    flat_permission_results.push(kc_permission)
                                }
                            }
                        }
                    }
                }
            }
        }

        // check if we have any permissions in Ivenza we didn't find in keycloak.
        let mut missing = 0;
        for permission in &source_permissions {
            if !flat_permission_results.iter().any(|pr| pr.eq(&permission)) {
                missing += 1;
                println!(
                    "\x1b[31m✘\x1b[0m {} for {}",
                    permission.permission, permission.role
                );
            } else {
                println!(
                    "\x1b[32m✔\x1b[0m {} for {}",
                    permission.permission, permission.role
                );
            }
        }

        // If we have missing items, print out the number of missing permissions.
        if missing > 0 {
            println!(
                "\x1b[31m✘ Missing {}/{} roles\x1b[0m",
                missing,
                source_permissions.len()
            );
        } else {
            // Great success!
            println!("\x1b[32m✔\x1b[0m All permissions found");
        }

        // Check that we didn´t add too many permissions to keycloak.
        // Note, this might not always be invalid, as de demo database in Ivenza also appears to
        // have invalid configurations. We should at least validate these manually.
        if &source_permissions.len() < &flat_permission_results.len() {
            let additional_permissions: Vec<Permission> = flat_permission_results
                .iter()
                .filter(|p| !source_permissions.iter().any(|sp| sp.eq(p)))
                .cloned()
                .collect();
            let mut validated_permissions: Vec<Permission> = vec![];
            for additional_permission in additional_permissions {
                if validated_permissions
                    .iter()
                    .any(|p| p.eq(&additional_permission))
                {
                    continue;
                }
                validated_permissions.push(additional_permission.clone());

                println!(
                    "\x1b[33m⚠\x1b[0m {} for {} was not found in ivenza",
                    additional_permission.permission, additional_permission.role
                );
                let sublevel_permissions = source_permissions.iter().filter(|sp| {
                    sp.permission.to_lowercase().starts_with(
                        format!("{}.", additional_permission.permission.to_lowercase()).as_str(),
                    ) && sp
                        .role
                        .to_lowercase()
                        .eq(&additional_permission.role.to_lowercase())
                });
                for sublevel_permission in sublevel_permissions {
                    println!(
                        "\tFOUND {},{}",
                        sublevel_permission.role, sublevel_permission.permission
                    );
                }
            }
        }
        Ok(())
    }
}

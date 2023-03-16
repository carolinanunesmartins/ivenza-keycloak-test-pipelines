use super::IvenzaClient;
use super::KeycloakClient;
use crate::models::ivenza::PermissionUtilities;
use crate::models::ivenza::*;
use crate::models::keycloak::{
    CreatePermissionRequest, DecisionStrategy, LogicType, PermissionResponse, PermissionType,
};
use crate::services::utility::{GroupBy, PushUnique};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

const STANDALONE_SCOPES: [&'static str; 7] = [
    "export",
    "import",
    "delete",
    "edit",
    "create",
    "recreate",
    "reschedule",
];

pub struct PermissionSyncer {}

impl PermissionSyncer {
    /// Determines which permissions are expected in keycloak, based on the permissions available
    /// in Ivenza.
    /// For each permission is determined which role is granted certain scopes to a particular
    /// resource.
    pub async fn sync() -> Result<(), Box<dyn Error>> {
        // Get all the permissions from Ivenza, this is our source of truth.
        let ivenza_permissions: Vec<Permission> = IvenzaClient::get_permissions();

        // Determine all the posible scopes from Ivenza.
        let scopes = IvenzaClient::determine_scopes();

        // Construct mapping from Ivenza permissions to permissions in keycloak.
        // A permission in keycloak is typically represented by a name (human readable), a
        // resource (manage.orders), scopes (edit, create, delete, etc...) and policies (Has
        // 'Developer' role) for example.
        let ivenza_permission_mappings = ivenza_permissions
            // Group permission names based on roles
            .ordered_group_by(|p| &p.permission, |a, b| a.role.eq(&b.role))
            .iter()
            .fold(
                HashMap::new(),
                |result: HashMap<String, PermissionMap>, permission_group| {
                    Self::get_permission_mappings(&scopes, result, permission_group)
                },
            )
            .correct_key_namings(&scopes);

        // Construct a new keycloak client.
        let mut keycloak_client = KeycloakClient::new();

        // Get all the known permissions from keycloak.
        let keycloak_permissions = keycloak_client.get_permissions().await?;

        // Determine which permissions are missing from keycloak based on the permission mappings
        // we constructed earlier.
        let missing_permissions =
            Self::determine_missing_permissions(&ivenza_permission_mappings, &keycloak_permissions);

        println!("Missing {} permissions", missing_permissions.len());

        // check if we need further processing.
        if missing_permissions.is_empty() {
            return Ok(());
        }

        // Get the currently known policies from keycloak, we will use these to assign to new
        // permissions later.
        let keycloak_policies = keycloak_client.get_policies().await?;

        // Get all the currently known scopes from keycloak. We will use these to assign to new
        // permissions alter.
        let keycloak_scopes = keycloak_client.get_scopes().await?;
        // Get all the currently known resource from keycloak. We will use these to a ssign to new
        // permissions later.
        let keycloak_resources = keycloak_client.get_resources().await?;

        // For each missing permission
        for missing_permission in missing_permissions {
            // Determine which resources should be assigned to the newly constructed permission and
            // select their IDs.
            let resource = keycloak_resources
                .iter()
                .find(|kr| kr.name.eq(&missing_permission.1.resource))
                .map(|kr| kr.id);

            // Determine which scopes should be assigned to the newly constructed permission and
            // select their IDs
            let scopes = keycloak_scopes
                .iter()
                .filter(|kc| {
                    missing_permission.1.scopes.contains(&kc.name)
                        && ivenza_permissions.iter().any(|p| {
                            p.permission
                                .eq(&format!("{}.{}", missing_permission.1.resource, kc.name))
                        })
                })
                .map(|kc| kc.id)
                .collect::<Vec<Uuid>>();

            // Determine which policies should be assigned to the newly constructed permission and
            // select their IDs
            let policies = keycloak_policies
                .iter()
                .filter(|kp| {
                    missing_permission.1.roles.iter().any(|ir| {
                        kp.name
                            .replace("Has ", "")
                            .replace(" role", "")
                            .to_lowercase()
                            .eq(&ir.to_lowercase())
                    })
                })
                .map(|kc| kc.id)
                .collect::<Vec<Uuid>>();

            if let Some(conflict_permission) = keycloak_permissions
                .iter()
                .find(|item| item.name.eq(missing_permission.0))
            {
                println!("Deleting conflict permissin '{}'", conflict_permission.name);
                keycloak_client
                    .delete_permission(&conflict_permission.id)
                    .await?;
            }
            println!(
                "Inserting permission '{}' for {} roles and {} scopes into keycloak",
                missing_permission.0,
                &policies.len(),
                &scopes.len()
            );

            // If we were able to determine a resource for this permission.
            if let Some(res) = resource {
                // create a new request to insert a permission into keycloak.
                let permission = CreatePermissionRequest {
                    name: missing_permission.0.to_string(),
                    description: missing_permission.0.to_string(),
                    r#type: PermissionType::SCOPE,
                    logic: LogicType::POSITIVE,
                    decision_strategy: DecisionStrategy::AFFIRMATIVE,
                    resources: vec![res],
                    policies,
                    scopes,
                };

                // Insert eh permission into keycloak.
                keycloak_client.insert_permission(&permission).await?;
            } else {
                // This is not good, apparently we have a resource missing in keycloak. This should
                // be fixed first.
                println!(
                    "Resource {} not found in keycloak for permission {}",
                    missing_permission.1.resource, missing_permission.0
                )
            }
        }
        Ok(())
    }

    /// Compares the permission mappings from Ivenza, and the currently known permissions from
    /// keycloak and determines which permissions are missing from keycloak.
    fn determine_missing_permissions<'a>(
        ivenza_permission_mappings: &'a BTreeMap<String, PermissionMap>,
        keycloak_permissions: &'a Vec<PermissionResponse>,
    ) -> Vec<(&'a String, &'a PermissionMap)> {
        let missing_permissions: Vec<(&String, &PermissionMap)> =
            ivenza_permission_mappings.iter().filter(|ipm|
                !keycloak_permissions
                    .iter()
                    .any(|kp|
                        kp.associated_resources.is_some() &&
                            // See if we have permissions that are already assigned to this resource
                            kp.associated_resources.as_ref().unwrap().iter().any(|ar| ar.name.eq(&ipm.1.resource)) &&
                            // which have every role assigned
                            ipm.1.roles.iter().all(|ipmr| kp.associated_role_policies.is_some() &&
                                kp.associated_role_policies.as_ref().unwrap().iter().any(|arp|
                                    arp.name.to_lowercase().contains(ipmr.to_lowercase().as_str())
                                )) &&
                            // which have every scope assigned
                            ipm.1.scopes.iter().all(|ipms| kp.associated_scopes.is_some() &&
                                kp.associated_scopes.as_ref().unwrap().iter().any(|ass| ass.name.to_lowercase().eq(&ipms.to_lowercase())))
                    )
            ).collect();
        missing_permissions
    }

    /// Construct mapping from Ivenza permissions to permissions in keycloak.
    /// A permission in keycloak is typically represented by a name (human readable), a
    /// resource (manage.orders), scopes (edit, create, delete, etc...) and policies (Has
    /// 'Developer' role) for example.
    fn get_permission_mappings(
        scopes: &Vec<String>,
        mut result: HashMap<String, PermissionMap>,
        permission_group: (&&String, &Vec<&Permission>),
    ) -> HashMap<String, PermissionMap> {
        // Get the name of the resource based on the ivena permission name.
        let resource = PermissionUtilities::extract_resource_scope(permission_group.0, &scopes);
        println!(
            "Permission {} mapped to resource {:?}",
            permission_group.0, resource
        );
        // Determine which roles are allowed to use this resource.
        let allowed_roles: Vec<String> = permission_group
            .1
            .iter()
            .map(|pgr| pgr.role.to_string())
            .collect();

        // Check if a previous iteration already filled this position.
        let mut entry_key = resource.0.to_string();
        if let Some(entry) = result.get(resource.0) {
            let matching =
                   // check that all allowed roles are available in the existing mapping
                   allowed_roles.iter().zip(&entry.roles).filter(|&(a, b)| a.eq(b)).count() ==
                       // and all the roles in the existing mapping also are available in this mapping.
                       // this should return a bidirectional match.
                   entry.roles.iter().zip(&allowed_roles).filter(|&(a, b)| a.eq(b)).count();
            if matching && !resource.1.is_empty() {
                entry_key = permission_group.0.to_string();
            }
        }

        // Get the Hashmap entry based on the given key.
        // If it does not exist, construct a new one.
        let entry = result.entry(entry_key).or_insert(PermissionMap {
            resource: resource.0.to_string(),
            scopes: resource.1.clone(),
            roles: allowed_roles.clone(),
        });
        // Append any missing scoped to the entry.
        for scope in resource.1 {
            entry.scopes.push_unique(&scope);
        }
        result
    }

    /// Converts a resource name from Ivenza to human readable names
    /// example 1 : manage.orders will become 'Can manage orders'
    /// example 2 : manage.orders.edit will become 'Can edit orders'
    /// example 3 : manage.orders.export.PNG will become 'Can export orders to PNG'
    fn permission_to_human_readable(
        permission_group: (&String, &PermissionMap),
        resource: (&str, Vec<String>),
    ) -> String {
        let scope = match resource.1.first() {
            Some(scope) => scope,
            None => "",
        };
        // top level resources
        if scope.is_empty() {
            return match resource.0.contains("manage.") {
                true => format!("Can manage {}", resource.0.replace("manage.", "")),
                false => resource.0.to_string(),
            };
        }

        let name = match permission_group.0.contains("export.") {
            // The permission is an export permission.
            true => format!(
                "Can export {} to {}",
                resource.0.replace("manage.", ""),
                scope.replace("export.", "")
            ),
            _ => match STANDALONE_SCOPES.contains(&scope) {
                // This is a stand alone scope, can be resolved to Can edit, can delete, can
                // recreate, etc....
                true => format!("Can {} {}", scope, resource.0.replace("manage.", "")),
                false => {
                    // otherwise, we can't determine something beautifull, so keep at can manage
                    // [SCOPE] on [RESOURCE].
                    return format!(
                        "Can manage {} on {}",
                        scope,
                        resource.0.replace("manage.", "")
                    );
                }
            },
        };
        name
    }
}

#[derive(Debug)]
struct PermissionMap {
    resource: String,
    scopes: Vec<String>,
    roles: Vec<String>,
}

impl Clone for PermissionMap {
    fn clone(&self) -> Self {
        Self {
            resource: self.resource.clone(),
            scopes: self.scopes.clone(),
            roles: self.roles.clone(),
        }
    }
}

trait PermissionNaming {
    fn correct_key_namings(&self, scopes: &Vec<String>) -> BTreeMap<String, PermissionMap>;
}

impl PermissionNaming for HashMap<String, PermissionMap> {
    fn correct_key_namings(&self, scopes: &Vec<String>) -> BTreeMap<String, PermissionMap> {
        let mut result: BTreeMap<String, PermissionMap> = BTreeMap::new();
        for row in self {
            let resource_with_scope = PermissionUtilities::extract_resource_scope(row.0, scopes);
            let name = PermissionSyncer::permission_to_human_readable(row, resource_with_scope);
            result.insert(name, row.1.clone());
        }
        result
    }
}

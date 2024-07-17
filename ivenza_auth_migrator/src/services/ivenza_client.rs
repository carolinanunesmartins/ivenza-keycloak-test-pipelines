use crate::data;
use crate::models::ivenza::*;
use sqlx::mysql::MySqlPool;
use crate::services::utility::PushUnique;
use regex::Regex;
use std::collections::HashMap;

use super::ROOT_LEVEL_SCOPE;

const IVENZA_PERMISSION_REGEX: &str = r"(?m)(.+?)\.((?:edit\.|export.|collada.|reschedule.|search.|filters.|import.|details.mod|kmz.)?[^.]+)$";

pub struct IvenzaClient
{
    my_sql_pool: MySqlPool,
}

impl IvenzaClient {

    pub async fn new() -> Self {
        let my_sql_pool = data::establish_connection().await;
        Self {
            my_sql_pool
        }
    }

    /// Get's all the roles from Ivenza
    pub async fn get_roles(&self) -> Vec<Role> {
        sqlx::query_as::<_, Role>("SELECT * FROM UserRoles").fetch_all(&self.my_sql_pool).await.expect("error loading roles")
    }

    pub async fn get_users(&self) -> Vec<User> {
        sqlx::query_as::<_, User>("SELECT * FROM Users WHERE state != 'DISABLED'").fetch_all(&self.my_sql_pool).await.expect("error loading users")
    }

    /// Gets all permissions from Ivenza.
    pub async fn get_permissions(&self) -> Vec<Permission> {
        let roles = self.get_roles().await;
        // get all the known permission, using the connection we just established
        let ivenza_permissions = sqlx::query_as::<_, Permission>("SELECT * FROM UserRolePermissions").fetch_all(&self.my_sql_pool).await.expect("error loading permissions");
        // filter out permissions for which no role exists.
        // There is no hard foreign key constraint in the database
        // So we can end up with permissions, that don't have a parent role in Ivenza
        ivenza_permissions
            .into_iter()
            .map(|mut p| {
                // We will only work with lowercase permissions, as keycloak doesn't like scopes
                // having two similar values either export.XLS or export.xls. The last item will
                // win.
                p.permission = p.permission.to_lowercase();
                p
            })
            .filter(|p| roles.iter().any(|r| p.role.eq(&r.name)))
            .collect()
    }

    /// Determines all the possible scopes based on the permissions in Ivenza.
    pub async fn determine_scopes(&self) -> Vec<String> {
        let permissions = self.get_permissions().await;
        let regex = Regex::new(IVENZA_PERMISSION_REGEX).unwrap();

        // extract resources from the ivenza permissions
        let mut scopes = permissions
            .iter() // iterator over all permissions.
            .filter(|&perm| perm.filter_scopes(&permissions)) // filter out some of the permissions that don't apply for a scope
            .map(|permission| regex.captures(permission.permission.as_str())) // perform a regex match
            .filter(|capture| capture.is_some()) // filter successful matches
            .map(|capture| capture.unwrap()[2].to_string()) // get the second tuple value from the match, this contains the scope.
            .filter(|scope| !scope.ends_with("address")) // we don't want the address scopes
            .fold(vec![], |mut result, capture| result.push_unique(&capture)); // fold all values into an array and push only unique values (distinct).
        scopes.push_unique(ROOT_LEVEL_SCOPE);
        scopes
    }

    /// Determines all the possible resources within Ivenza and groups all related possible scopes
    /// to it.
    pub fn determine_resources<'a>(
        permissions: &'a Vec<Permission>,
        scopes: &'a Vec<String>,
    ) -> HashMap<&'a str, Vec<String>> {
        // extract resources from the ivenza permissions
        let resources = permissions
            .iter() // iterator over all permissions.
            .map(|permission| permission.extract_resource_scope(&scopes))
            .filter(|perm| perm.0 != "manage") // Filter out the manage resource...it's the only one we don't care about.
            .fold(HashMap::new(), |mut result, capture| {
                let entry = result.entry(capture.0).or_insert(capture.1.clone());
                for val in capture.1 {
                    if permissions
                        .iter()
                        .any(|p| p.permission.eq(&format!("{}.{}", capture.0, val)))
                    {
                        entry.push_unique(&val);
                    }
                }
                result
            }); // fold all values into an array and push only unique values (distinct).
        resources
    }
}

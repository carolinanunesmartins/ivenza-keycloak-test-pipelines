use crate::data;
use crate::models::ivenza::*;
use crate::schema::UserRolePermissions::dsl::UserRolePermissions;
use crate::schema::Users::dsl::Users;
use crate::services::utility::PushUnique;
use diesel::prelude::*;
use regex::Regex;
use std::collections::HashMap;

const IVENZA_PERMISSION_REGEX: &str = r"(?m)(.+?)\.((?:edit\.|export.|collada.|reschedule.|search.|filters.|import.|details.mod|kmz.)?[^.]+)$";

pub struct IvenzaClient;

impl IvenzaClient {
    /// Get's all the roles from Ivenza
    pub fn get_roles() -> Vec<Role> {
        // Retrieve the known roles from Ivenza
        // let db_connection = &mut data::establish_connection();
        // let ivenza_roles: Vec<Role> = UserRoles
        //     .load::<Role>(db_connection)
        //     .expect("error loading roles");
        //  Get roles based on role column in permissions
        let permissions = Self::get_permissions();
        let ivenza_roles = permissions
            .iter()
            .map(|p| &p.role)
            .fold(vec![], |mut result, capture| result.push_unique(&capture)) // fold all values into an array and push only unique values (distinct).
            .iter()
            .map(|r| Role {
                id: 0,
                name: r.to_string(),
                display_name: r.to_string(),
            })
            .collect::<Vec<Role>>();
        ivenza_roles
    }

    pub fn get_users() -> Vec<User> {
        // Retrieve all known users from ivenza which don't have the delihome/uniconcreation e-mail
        // address
        let db_connection = &mut data::establish_connection();
        let ivenza_users: Vec<User> = Users
            .load::<User>(db_connection)
            .expect("error loading users");
        ivenza_users
    }

    /// Gets all permissions from Ivenza.
    pub fn get_permissions() -> Vec<Permission> {
        // Retrieve the known roles from Ivenza
        let db_connection = &mut data::establish_connection();
        let ivenza_permissions: Vec<Permission> = UserRolePermissions
            .load::<Permission>(db_connection)
            .expect("error loading permissions");
        ivenza_permissions
    }

    /// Determines all the possible scopes based on the permissions in Ivenza.
    pub fn determine_scopes() -> Vec<String> {
        let permissions = Self::get_permissions();
        let regex = Regex::new(IVENZA_PERMISSION_REGEX).unwrap();

        // extract resources from the ivenza permissions
        let scopes = permissions
            .iter() // iterator over all permissions.
            .filter(|&perm| perm.filter_scopes(&permissions)) // filter out some of the permissions that don't apply for a scope
            .map(|permission| regex.captures(permission.permission.as_str())) // perform a regex match
            .filter(|capture| capture.is_some()) // filter successful matches
            .map(|capture| capture.unwrap()[2].to_string()) // get the second tuple value from the match, this contains the scope.
            .filter(|scope| !scope.ends_with("Address")) // we don't want the address scopes
            .fold(vec![], |mut result, capture| {
                result.push_unique(&capture.to_lowercase())
            }); // fold all values into an array and push only unique values (distinct).
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

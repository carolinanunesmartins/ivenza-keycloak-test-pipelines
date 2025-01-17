use std::cmp::Ordering;
use sqlx::FromRow;

/// Structs representing tables within the Ivenza database.
#[derive(FromRow, Debug, Clone)]
pub struct Permission {
    pub role: String,
    pub permission: String,
}

#[derive(FromRow, Debug)]
pub struct Role {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "displayName")]
    pub display_name: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct User {
    #[sqlx(rename = "ID")]
    pub id: i32,
    #[sqlx(rename = "loginName")]
    pub login_name: String,
    pub role: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub domain: Option<String>,
    pub state: String,
}
// Compare trait implementations to compare permissions for equality.
impl Eq for Permission {}

impl PartialEq<Self> for Permission {
    fn eq(&self, other: &Self) -> bool {
        // we check case insensitive for the in ivenza for roles and permissions
        self.permission.eq(&other.permission) && self.role.to_lowercase().eq(&other.role.to_lowercase())
    }
}

impl PartialOrd<Self> for Permission {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.permission.len() > other.permission.len() {
            return Some(Ordering::Greater);
        }
        if self.permission.len() < other.permission.len() {
            return Some(Ordering::Less);
        }
        Some(Ordering::Equal)
    }
}

impl Ord for Permission {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Permission {
    /// Determines whether permissions have scopes included or not.
    /// For example
    /// `manage.orders` will not have a scope included as this only results in a resource
    /// `manage.order.edit` will have the `edit` scope included and will be a part of the
    /// permissions filterd by possible scopes.
    pub fn filter_scopes(&self, permissions: &Vec<Permission>) -> bool {
        // we are only interested in items with more then 2 levels
        (self.permission.split(".").collect::<Vec<&str>>().len() > 2 &&

            // which don't have nested types yet.
            !permissions.iter().any(|other|
                other.permission.as_str().starts_with(format!("{}.",self.permission).as_str()) &&
                    other.permission.len() > self.permission.len())) ||
            // or start with design.
            self.permission.starts_with("design.")
    }

    /// Extracts the expected resouce and scopes based on the given permission.
    pub fn extract_resource_scope(&self, scopes: &Vec<String>) -> (&str, Vec<String>) {
        PermissionUtilities::extract_resource_scope(&self.permission, scopes)
    }
}

/// Helper class for Permissions
pub struct PermissionUtilities;
impl PermissionUtilities {
    /// extracts resouces, with scopes from the given permission
    pub fn extract_resource_scope<'a>(
        resource_name: &'a str,
        scopes: &Vec<String>,
    ) -> (&'a str, Vec<String>) {
        let coupled_scopes: Vec<String> = scopes
            .iter()
            .filter(|&scope| resource_name.ends_with(format!(".{}", &scope).as_str()))
            .cloned()
            .collect();
        match coupled_scopes.len() > 0 &&
            // there is a special treatment for measure preferences at semi root manage level, because there is also a measurePreferences scope for shop.personal.details.
            resource_name != "manage.measurepreferences" &&
            resource_name != "manage.productlines"
        {
            true => {
                let pat = format!(".{}", &coupled_scopes[0]);
                let index = resource_name.find(&pat);
                match index {
                    Some(idx) => (&resource_name[..idx], coupled_scopes),
                    None => (resource_name, coupled_scopes),
                }
            }
            false => (resource_name, vec![]),
        }
    }
}

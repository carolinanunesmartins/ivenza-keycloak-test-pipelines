use super::ivenza::User;
use serde::{Deserialize, Serialize};
use std::vec;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub composite: bool,
    #[serde(rename = "clientRole")]
    pub client_role: bool,
    #[serde(rename = "containerId")]
    pub container_id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub id: Uuid,
    #[serde(rename = "username")]
    pub user_name: String,
    #[serde(rename = "firstName", default)]
    pub first_name: String,
    #[serde(rename = "lastName", default)]
    pub last_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateRoleRequest {
    name: String,
    description: String,
}

impl CreateRoleRequest {
    pub fn new(name: &str, description: &str) -> Self {
        CreateRoleRequest {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CreateUserRequest {
    #[serde(rename = "username")]
    pub user_name: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    enabled: bool,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    credentials: Option<Vec<Credentials>>,
    groups: Option<Vec<String>>,
    attributes: Option<UserAttribute>,
}

#[derive(Serialize, Debug)]
pub struct UserAttribute {
    #[serde(rename = "ivenzaId")]
    ivenza_id: Vec<String>,
    domain: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Credentials {
    #[serde(rename = "type")]
    pub credential_type: CredentialType,
    pub value: String,
    pub temporary: bool,
}

#[derive(Serialize, Debug)]
pub enum CredentialType {
    Password,
    // What else?
}

impl From<&User> for CreateUserRequest {
    fn from(user: &User) -> Self {
        CreateUserRequest {
            user_name: user.login_name.to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            enabled: true,
            email: user.email.to_string(),
            email_verified: true,
            groups: Some(vec![]),
            attributes: Some(UserAttribute {
                ivenza_id: vec![user.id.to_string()],
                domain: vec![user.domain.clone().unwrap_or("".to_string()).to_string()],
            }),
            credentials: Some(vec![Credentials {
                credential_type: CredentialType::Password,
                value: user.password.to_string(),
                temporary: false,
            }]),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct AssignRoleRequest {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub composite: bool,
    #[serde(rename = "clientRole")]
    pub client_role: bool,
    #[serde(rename = "containerId")]
    pub container_id: Uuid,
}

impl From<&RoleResponse> for AssignRoleRequest {
    fn from(role: &RoleResponse) -> Self {
        AssignRoleRequest {
            id: role.id,
            name: role.name.to_string(),
            description: role.description.to_string(),
            composite: role.composite,
            client_role: false,
            container_id: role.container_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct ScopeResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "iconUri")]
    pub icon_uri: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateScopeRequest {
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iconUri")]
    pub icon_uri: String,
}

impl CreateScopeRequest {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            display_name: name.to_string(),
            icon_uri: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PolicyRoles {
    pub roles: String,
}

impl PolicyRoles {
    pub fn get_roles(&self) -> Option<Vec<PolicyRole>> {
        let result: Result<Vec<PolicyRole>, _> = serde_json::from_str(self.roles.as_str());
        match result {
            Ok(roles) => Some(roles),
            _ => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PolicyResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub logic: LogicType,
    #[serde(rename = "decisionStrategy")]
    pub decision_strategy: DecisionStrategy,
    pub config: PolicyRoles,
}

/// Payload to create a new permission in Keycloak.
/// ```json {
///     "resources":["7d8053a4-13e5-40b6-9c2f-fc76982faf5d"],
///     "policies":["611756fa-33a1-4095-92f6-4a004e63962d","469110a8-aed6-40ff-a337-ce0acdf8a8da","d01c0669-2bde-4c64-97d5-3c93739f5fdc"],
///     "scopes":["1a7e1984-f781-4bd1-a742-a5b82b94d5dc"],
///     "decisionStrategy":"AFFIRMATIVE",
///     "name":"Can cancel order",
///     "description":"Can cancel order"
/// ```}
#[derive(Serialize, Debug)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: PermissionType,
    pub logic: LogicType,
    #[serde(rename = "decisionStrategy")]
    pub decision_strategy: DecisionStrategy,
    pub resources: Vec<Uuid>,
    pub policies: Vec<Uuid>,
    pub scopes: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DecisionStrategy {
    AFFIRMATIVE,
    UNANIMOUS,
    CONSENSUS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolicyRole {
    pub id: Uuid,
    pub required: bool,
}

#[derive(Serialize, Debug)]
pub struct CreateRoleBasedPolicyRequest {
    pub roles: Vec<PolicyRole>,
    pub name: String,
    pub description: String,
    pub logic: LogicType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LogicType {
    POSITIVE,
    NEGATIVE,
}

impl CreateRoleBasedPolicyRequest {
    pub fn new(keycloak_role: &RoleResponse) -> Self {
        let roles: Vec<PolicyRole> = vec![PolicyRole {
            id: keycloak_role.id,
            required: true,
        }];
        CreateRoleBasedPolicyRequest {
            roles,
            name: format!("Has {} role", keycloak_role.name),
            description: format!("Has {} role", keycloak_role.name),
            logic: LogicType::POSITIVE,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Owner {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceResponse {
    pub name: String,
    pub owner: Owner,
    #[serde(rename = "ownerManagedAccess")]
    pub owner_managed_access: bool,
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub uris: Vec<String>,
    pub icon_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {}

#[derive(Serialize, Deserialize)]
pub struct CreateResourceRequest {
    pub attributes: Attributes,
    pub scopes: Vec<ScopeResponse>,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub icon_uri: String,
    #[serde(rename = "ownerManagedAccess")]
    pub owner_managed_access: bool,
}

impl CreateResourceRequest {
    pub fn new(name: &str, scopes: Vec<&ScopeResponse>) -> Self {
        Self {
            name: name.to_string(),
            scopes: scopes
                .iter()
                .map(|&s| ScopeResponse {
                    id: s.id.clone(),
                    name: s.name.clone(),
                    display_name: s.display_name.clone(),
                    icon_uri: s.icon_uri.clone(),
                })
                .collect(),
            icon_uri: "".to_string(),
            attributes: Attributes {},
            r#type: "".to_string(),
            owner_managed_access: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: PermissionType,
    pub logic: LogicType,
    #[serde(rename = "decisionStrategy")]
    pub decision_strategy: DecisionStrategy,
    pub associated_role_policies: Option<Vec<AssociatedRolePolicies>>,
    pub associated_scopes: Option<Vec<AssociatedScope>>,
    pub associated_resources: Option<Vec<AssociatedResource>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PermissionType {
    #[serde(rename = "scope")]
    SCOPE,
    #[serde(rename = "resource")]
    RESOURCE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociatedScope {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociatedRolePolicies {
    pub id: Uuid,
    pub name: String,
}

impl AssociatedRolePolicies {
    pub fn get_role_name(&self) -> String {
        self.name
            .replace("Has ", "")
            .replace(" role", "")
            .to_lowercase()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssociatedResource {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
}

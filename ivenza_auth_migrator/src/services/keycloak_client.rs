use super::oidc_client::OidcClient;
use crate::models::ivenza::User;
use crate::models::keycloak::*;
use crate::services::utility;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

const ROLES_PATH: &str = "/roles";
const USERS_PATH: &str = "/users";
const AUTHORIZATION_HEADER: &str = "Authorization";
const AUTHORIZATION_BEARER_TOKEN: &str = "Bearer";
const ADMIN_BASE_URL_KEY: &str = "ADMIN_BASE_URL";
const CLIENT_ID_KEY: &str = "CLIENT_ID";
const JSON_CONTENT_TYPE: &str = "application/json";
const CONTENT_TYPE_HEADER: &str = "content-type";

#[derive(Debug)]
struct KeycloakError(String);
impl Error for KeycloakError {}

impl fmt::Display for KeycloakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
pub struct KeycloakClient {
    oidc_client: OidcClient,
    admin_base_url: String,
    client_id: String,
    http_client: Client<HttpConnector>,
}

impl KeycloakClient {
    /// Keycloak client constructor
    pub fn new() -> Self {
        let admin_base_url = env::var(ADMIN_BASE_URL_KEY).expect("ADMIN_BASE_URL not set.");
        let client_id = env::var(CLIENT_ID_KEY).expect("CLIENT_ID not set.");
        Self {
            oidc_client: OidcClient::new(),
            admin_base_url,
            client_id,
            http_client: Client::new(),
        }
    }

    /// Gets all the realm roles from Keycloak.
    pub async fn get_roles(&mut self) -> Result<Vec<RoleResponse>, Box<dyn Error>> {
        // construct the roles endpoint url.
        let roles_endpoint = format!("{}{}", self.admin_base_url, ROLES_PATH);
        return self.http_get(roles_endpoint).await;
    }

    /// Gets all the realm roles from Keycloak.
    pub async fn get_users(&mut self) -> Result<Vec<UserResponse>, Box<dyn Error>> {
        // construct the roles endpoint url.
        let roles_endpoint = format!("{}{}", self.admin_base_url, USERS_PATH);
        return self.http_get(roles_endpoint).await;
    }

    /// Gets all the client scopes from Keycloak.
    pub async fn get_scopes(&mut self) -> Result<Vec<ScopeResponse>, Box<dyn Error>> {
        // construct the scopes endpoint url.
        let scopes_endpoint = format!(
            "{}/clients/{}/authz/resource-server/scope?max=1000",
            self.admin_base_url, self.client_id
        );
        return self.http_get(scopes_endpoint).await;
    }

    /// Gets all the resources from Keycloak.
    pub async fn get_resources(&mut self) -> Result<Vec<ResourceResponse>, Box<dyn Error>> {
        // construct the scopes endpoint url.
        let scopes_endpoint = format!(
            "{}/clients/{}/authz/resource-server/resource?max=10000",
            self.admin_base_url, self.client_id
        );
        return self.http_get(scopes_endpoint).await;
    }

    /// Gets all permissions from Keycloak
    /// Note that getting the permissions also gets all underling related scopes, resources and
    /// policies assigned to this permission.
    pub async fn get_permissions(&mut self) -> Result<Vec<PermissionResponse>, Box<dyn Error>> {
        // construct the scopes endpoint url.
        let scopes_endpoint = format!(
            "{}/clients/{}/authz/resource-server/permission?max=10000",
            self.admin_base_url, self.client_id
        );

        // Get the permissions
        let permissions: Result<Vec<PermissionResponse>, Box<dyn Error>> =
            self.http_get(scopes_endpoint).await;
        match permissions {
            // If we got a successfull response from Keycloak.
            Ok(mut permission_list) => {
                // iterate over all the permissions
                for permission in &mut permission_list {
                    //get the scopes assigned to this permission.
                    let associated_scopes: Result<Vec<AssociatedScope>, Box<dyn Error>> = self
                        .http_get(format!(
                            "{}/clients/{}/authz/resource-server/policy/{}/scopes?max=10000",
                            self.admin_base_url, self.client_id, permission.id
                        ))
                        .await;
                    if associated_scopes.is_ok() {
                        // successfull response, set the value to the permission
                        permission.associated_scopes = Some(associated_scopes.unwrap());
                    }

                    //get the resources assigned to this permission
                    let associated_resources: Result<Vec<AssociatedResource>, Box<dyn Error>> =
                        self.http_get(format!(
                            "{}/clients/{}/authz/resource-server/policy/{}/resources?max=10000",
                            self.admin_base_url, self.client_id, permission.id
                        ))
                        .await;
                    if associated_resources.is_ok() {
                        // Successfull response, set the value to the permission.
                        permission.associated_resources = Some(associated_resources.unwrap());
                    }

                    //get the policies assigned to this permission
                    let associated_role_policies: Result<
                        Vec<AssociatedRolePolicies>,
                        Box<dyn Error>,
                    > = self
                        .http_get(format!(
                            "{}/clients/{}/authz/resource-server/policy/{}/associatedPolicies",
                            self.admin_base_url, self.client_id, permission.id
                        ))
                        .await;
                    if associated_role_policies.is_ok() {
                        // successfull response, set the value
                        permission.associated_role_policies =
                            Some(associated_role_policies.unwrap());
                    }
                }
                // return our result.
                Ok(permission_list)
            }
            _ => permissions,
        }
    }

    /// Gets all the client scopes from Keycloak.
    pub async fn get_policies(&mut self) -> Result<Vec<PolicyResponse>, Box<dyn Error>> {
        // construct the roles endpoint url.
        let policy_endpoint = format!(
            "{}/clients/{}/authz/resource-server/policy?permission=false",
            self.admin_base_url, self.client_id
        );
        return self.http_get(policy_endpoint).await;
    }

    /// Inserts a resource into keycloak.
    pub async fn insert_resource(
        &mut self,
        resource_name: &str,
        assigned_keycloak_scopes: Vec<&ScopeResponse>,
    ) -> Result<(), Box<dyn Error>> {
        // construct the scopes endpoint url.
        let resource_endpoint = format!(
            "{}/clients/{}/authz/resource-server/resource",
            self.admin_base_url, self.client_id
        );

        // construct the request.
        let request = CreateResourceRequest::new(resource_name, assigned_keycloak_scopes);
        // Post to keycloak.
        _ = self.http_post(resource_endpoint, &request).await;
        Ok(())
    }

    pub async fn insert_role(
        &mut self,
        name: &str,
        description: &str,
    ) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let roles_endpoint = format!("{}{}", self.admin_base_url, ROLES_PATH);
        // construct the request.
        let request = CreateRoleRequest::new(name, description);
        // post to keycloak.
        _ = self.http_post(roles_endpoint, &request).await;
        Ok(())
    }

    pub async fn insert_user(
        &mut self,
        user: &User,
        role: &RoleResponse,
    ) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let users_endpoint = format!("{}{}", self.admin_base_url, USERS_PATH);
        // construct the request.
        let request: CreateUserRequest = user.into();
        // post to keycloak.
        let response = self.http_post(users_endpoint, &request).await;
        match response {
            // The created response returns the location of the user where we can set the roles too
            Ok(user_management_location) => {
                // construct the role mapping endpoint for our newly created user
                let assign_role_url = format!("{user_management_location}/role-mappings/realm");
                // map our role to a assign role request
                let assign_role_request: Vec<AssignRoleRequest> = vec![role.into()];
                // Perform the call to assign the role to the newly created user.
                match self.http_post(assign_role_url, &assign_role_request).await {
                    // Check the response, throw otherwise
                    Ok(_) => Ok(()),
                    _ => Err(Box::new(KeycloakError(
                        "Unable to assign role to user in keycloak".into(),
                    ))),
                }
            }
            // We were unable to create the user in keycloak, let's throw an exception.
            _ => Err(Box::new(KeycloakError(
                "Unable to insert user into keycloak".into(),
            ))),
        }
    }

    /// Inserts a role based policy for the given keycloak role to keycloak.
    pub async fn insert_role_based_policy(
        &mut self,
        keycloak_role: &RoleResponse,
    ) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let policy_endpoint = format!(
            "{}/clients/{}/authz/resource-server/policy/role",
            self.admin_base_url, self.client_id
        );
        // construct the request.
        let request = CreateRoleBasedPolicyRequest::new(&keycloak_role);
        // post to keycloak.
        _ = self.http_post(policy_endpoint, &request).await;
        Ok(())
    }

    /// Inserts a scope with the given name into keycloak.
    pub async fn insert_scope(&mut self, scope_name: &str) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let endpoint = format!(
            "{}/clients/{}/authz/resource-server/scope",
            self.admin_base_url, self.client_id
        );
        // construct the request.
        let request = CreateScopeRequest::new(scope_name);
        // post to keycloak.
        _ = self.http_post(endpoint, &request).await;
        Ok(())
    }

    /// Inserts a permission into keycloak.
    pub async fn insert_permission(
        &mut self,
        request: &CreatePermissionRequest,
    ) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let scopes_endpoint = format!(
            "{}/clients/{}/authz/resource-server/permission",
            self.admin_base_url, self.client_id
        );
        // post to keycloak
        _ = self.http_post(scopes_endpoint, &request).await;
        Ok(())
    }

    pub async fn delete_permission(&mut self, id: &Uuid) -> Result<(), Box<dyn Error>> {
        // construct the roles endpoint url.
        let delete_permission_endpoint = format!(
            "{}/clients/{}/authz/resource-server/permission/scope/{}",
            self.admin_base_url, self.client_id, id
        );
        // post to keycloak
        _ = self.http_delete(delete_permission_endpoint).await;
        Ok(())
    }

    /// Asynchronously performs a Http get request to the given endpoint and deserialized the JSON response to the
    /// given type T.
    async fn http_get<T>(&mut self, endpoint: String) -> Result<Vec<T>, Box<dyn Error>>
    where
        for<'de> T: Deserialize<'de>,
    {
        // Get an access_token authorize at the keycloak instance, or reuse the active one.
        let access_token = self.oidc_client.get_access_token().await?;

        // Build the request
        let req = Request::builder()
            .method(Method::GET)
            .uri(endpoint)
            .header(
                AUTHORIZATION_HEADER,
                format!("{} {}", AUTHORIZATION_BEARER_TOKEN, access_token),
            )
            .body(Body::empty())
            .expect("unable to build request");

        // Send the request and await the response.
        let mut resp = self.http_client.request(req).await?;

        // Check if this was successful.
        match resp.status() {
            StatusCode::OK => {
                // Great success! Now deserialize the response stream async and return deserialized instance;.
                Ok(utility::deserialize(&mut resp).await?)
            }
            _ => {
                // Oh-oh, something went wrong, log the response body and throw the exception.
                let _ = utility::print_response_body(&mut resp).await;
                panic!("Unable to retrieve items from Keycloak")
            }
        }
    }

    /// Serialized the given request instance to JSON and asynchronously performs a HTTP Post request to the given
    /// endpoint
    async fn http_post<T>(
        &mut self,
        endpoint: String,
        request: &T,
    ) -> Result<String, Box<dyn Error>>
    where
        T: Serialize,
    {
        // serialize the payload.
        let body =
            serde_json::to_string(&request).expect("Unable to serialize create role request");
        // Get an access_token authorize at the keycloak instance.
        let access_token = self.oidc_client.get_access_token().await?;
        // Build the request
        let req = Request::builder()
            .method(Method::POST)
            .uri(endpoint)
            .header(CONTENT_TYPE_HEADER, JSON_CONTENT_TYPE)
            .header(
                AUTHORIZATION_HEADER,
                format!("{} {}", AUTHORIZATION_BEARER_TOKEN, access_token),
            )
            .body(Body::from(body))
            .expect("unable to build request");

        // Send the request and await the response.
        let mut resp = self.http_client.request(req).await?;
        // Check if this was successful.
        match resp.status() {
            StatusCode::CREATED => {
                // Great success!
                match resp.headers().get("location") {
                    Some(location) => Ok(location.to_str().unwrap_or("").to_string()),
                    None => Ok("".to_string()),
                }
            }
            StatusCode::NO_CONTENT => Ok("".to_string()),
            _ => {
                // Oh-oh, something went wrong, log the response body and throw the exception.
                println!("{:?}", resp);
                let _ = utility::print_response_body(&mut resp).await;
                panic!("Unable to insert item in Keycloak")
            }
        }
    }

    async fn http_delete(&mut self, endpoint: String) -> Result<(), Box<dyn Error>> {
        // Get an access_token authorize at the keycloak instance.
        let access_token = self.oidc_client.get_access_token().await?;
        // Build the request
        let req = Request::builder()
            .method(Method::DELETE)
            .uri(endpoint)
            .header(
                AUTHORIZATION_HEADER,
                format!("{} {}", AUTHORIZATION_BEARER_TOKEN, access_token),
            )
            .body(Body::empty())
            .expect("unable to build request");

        // Send the request and await the response.
        let mut resp = self.http_client.request(req).await?;
        // Check if this was successful.
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => {
                // Oh-oh, something went wrong, log the response body and throw the exception.
                println!("{:?}", resp);
                let _ = utility::print_response_body(&mut resp).await;
                panic!("Unable to insert item in Keycloak")
            }
        }
    }
}

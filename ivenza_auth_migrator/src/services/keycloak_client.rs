use super::oidc_client::OidcClient;
use crate::models::keycloak::*;
use crate::services::utility;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

const ROLES_PATH: &str = "/roles";
const AUTHORIZATION_HEADER: &str = "Authorization";
const AUTHORIZATION_BEARER_TOKEN: &str = "Bearer";
const ADMIN_BASE_URL_KEY: &str = "ADMIN_BASE_URL";
const CLIENT_ID_KEY: &str = "CLIENT_ID";
const JSON_CONTENT_TYPE: &str = "application/json";
const CONTENT_TYPE_HEADER: &str = "content-type";

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
        return self.http_post(resource_endpoint, &request).await;
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
        return self.http_post(roles_endpoint, &request).await;
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
        return self.http_post(policy_endpoint, &request).await;
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
        return self.http_post(endpoint, &request).await;
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
        return self.http_post(scopes_endpoint, &request).await;
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
    async fn http_post<T>(&mut self, endpoint: String, request: &T) -> Result<(), Box<dyn Error>>
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
                Ok(())
            }
            _ => {
                // Oh-oh, something went wrong, log the response body and throw the exception.
                let _ = utility::print_response_body(&mut resp).await;
                panic!("Unable to insert item in Keycloak")
            }
        }
    }
}

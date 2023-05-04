use crate::services::utility;
use hyper::{Client, Method, Request, StatusCode};
use serde::Deserialize;
use std::env;
use urlencoding::encode;

const AUTHORITY_KEY: &str = "AUTHORITY";
const KEYCLOAK_CLIENT_ID_KEY: &str = "KEYCLOAK_CLIENT_ID";
const KEYCLOAK_ADMIN_USERNAME_KEY: &str = "KEYCLOAK_ADMIN_USERNAME";
const KEYCLOAK_ADMIN_PASSWORD_KEY: &str = "KEYCLOAK_ADMIN_PASSWORD";
const TOKEN_RELATIVE_PATH: &str = "/protocol/openid-connect/token";
const CONTENT_TYPE_HEADER: &str = "content-type";
const WWW_FORM_ENCODED_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
const PASSWORD_GRANT_TYPE: &str = "password";

pub struct OidcClient {
    access_token: Option<String>,
    authority: String,
    client_id: String,
    username: String,
    password: String,
}

impl OidcClient {
    // Oidc client constructor
    pub fn new() -> Self {
        // Get the OIDC server information from the environment variables.
        let authority = env::var(AUTHORITY_KEY).expect("AUTHORITY not set.");
        let client_id = env::var(KEYCLOAK_CLIENT_ID_KEY).expect("KEYCLOAK_CLIENT_ID not set.");
        let username =
            env::var(KEYCLOAK_ADMIN_USERNAME_KEY).expect("KEYCLOAK_ADMIN_USERNAME not set.");
        let password =
            env::var(KEYCLOAK_ADMIN_PASSWORD_KEY).expect("KEYCLOAK_ADMIN_PASSWORD not set.");

        // Return a newly constructed OIDC client.
        return Self {
            authority,
            client_id,
            username,
            password,
            access_token: None,
        };
    }

    /// Gets an access token from the OIDC server.
    pub async fn get_access_token(&mut self) -> Result<&String, Box<dyn std::error::Error>> {
        // If we already got an access token, return that
        if self.access_token.is_some() {
            return Ok(self.access_token.as_ref().unwrap());
        }

        // Format the token endpoint to retrieve the access token from.
        let token_endpoint = format!("{}{}", self.authority, TOKEN_RELATIVE_PATH);

        // construct the form data for the request
        let request_body = format!(
            "grant_type={}&client_id={}&username={}&password={}",
            PASSWORD_GRANT_TYPE,
            self.client_id,
            self.username,
            encode(&self.password)
        );

        // Build the request
        let req = Request::builder()
            .method(Method::POST)
            .uri(token_endpoint)
            .header(CONTENT_TYPE_HEADER, WWW_FORM_ENCODED_CONTENT_TYPE)
            .body(request_body.into())
            .expect("unable to build request");

        // Construct a HttpClient
        let client = Client::new();

        // Send the request and await the response.
        let mut resp = client.request(req).await?;

        // Check if this was successful.
        match resp.status() {
            StatusCode::OK => {
                // Great success! Now deserialize the response stream async and return the access_token.
                let parsed: TokenResponse = utility::deserialize(&mut resp).await?;
                self.access_token = Some(parsed.access_token);
                Ok(self.access_token.as_ref().unwrap())
            }
            _ => {
                // Oh-oh, something went wrong, log the response body and throw the exception.
                let _ = utility::print_response_body(&mut resp).await;
                panic!("Did not get a valid token from the OIDC client")
            }
        }
    }
}

#[derive(Deserialize)]
// Allow dead code, maybe some day we want to make use of the un-used values
#[allow(dead_code)]
struct TokenResponse {
    access_token: String,
    expires_in: i32,
    refresh_expires_in: i32,
    token_type: String,
    scope: String,
}

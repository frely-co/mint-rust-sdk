use reqwest::blocking::Client;
use thiserror::Error;
use std::collections::HashMap;

use crate::models::{
    AdminInitiateAuthRequest, AdminInitiateAuthResponse, SignUpRequest, SignUpResponse
};

/// CognitoClient for interacting with the Cognito mock API.
pub struct CognitoClient {
    base_url: String,
    client_id: String,
    client: Client,
}

const KEY: &str = "X-Amz-Target";
const CONTENT_TYPE: &str = "application/x-amz-json-1.1";

const AWS_COGNITO_IDENTITY_PROVIDER_SERVICE_SIGNUP: &str =
    "AWSCognitoIdentityProviderService.SignUp";

const AWS_COGNITO_IDENTITY_PROVIDER_SERVICE_ADMIN_INITIATE_AUTH: &str =
    "AWSCognitoIdentityProviderService.AdminInitiateAuth";

#[derive(Error, Debug)]
pub enum MINTError {
    #[error("Request failed with status: {0}")]
    HttpError(reqwest::StatusCode),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

impl MINTError {
    pub fn from_response(response: reqwest::blocking::Response) -> Self {
        MINTError::HttpError(response.status())
    }
}

impl CognitoClient {
    pub fn new(base_url: &str, client_id: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client_id: client_id.to_string(),
            client: Client::new(),
        }
    }

    /// Sign up a new user.
    pub fn sign_up(&self, username: &str, password: &str) -> Result<SignUpResponse, MINTError> {
        let payload = SignUpRequest {
            client_id: self.client_id.clone(),
            username: username.to_string(),
            password: password.to_string(),
        };

        let response = self
            .client
            .post(&self.base_url)
            .header(KEY, AWS_COGNITO_IDENTITY_PROVIDER_SERVICE_SIGNUP)
            .header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            Ok(response.json()?)
        } else {
            Err(MINTError::from_response(response))
        }
    }

    /// Authenticate a user.
    pub fn admin_initiate_auth(&self, username: &str, password: &str) -> Result<AdminInitiateAuthResponse, MINTError> {
        let auth_parameters = HashMap::from([
            ("USERNAME".to_string(), username.to_string()),
            ("PASSWORD".to_string(), password.to_string()),
        ]);

        let payload = AdminInitiateAuthRequest {
            client_id: self.client_id.clone(),
            auth_flow: "USER_PASSWORD_AUTH".to_string(),
            auth_parameters: Some(auth_parameters),
            client_metadata: None,
            user_pool_id: self.client_id.clone(), // TODO fix it
        };

        let response = self
            .client
            .post(&self.base_url)
            .header(KEY, AWS_COGNITO_IDENTITY_PROVIDER_SERVICE_ADMIN_INITIATE_AUTH)
            .header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            Ok(response.json()?)
        } else {
            Err(MINTError::from_response(response))
        }
    }
}

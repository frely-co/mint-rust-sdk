use reqwest::blocking::Client;
use thiserror::Error;

use crate::models::{AuthParameters, AuthRequest, AuthResponse, SignUpRequest};

/// CognitoClient for interacting with the Cognito mock API.
pub struct CognitoClient {
    base_url: String,
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
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    /// Sign up a new user.
    pub fn sign_up(&self, username: &str, password: &str) -> Result<(), MINTError> {
        let payload = SignUpRequest {
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
            Ok(())
        } else {
            Err(MINTError::from_response(response))
        }
    }

    /// Authenticate a user.
    pub fn authenticate(&self, username: &str, password: &str) -> Result<String, MINTError> {
        let payload = AuthRequest {
            auth_flow: "USER_PASSWORD_AUTH".to_string(),
            auth_parameters: AuthParameters {
                username: username.to_string(),
                password: password.to_string(),
            },
        };

        let response = self
            .client
            .post(&self.base_url)
            .header(KEY, AWS_COGNITO_IDENTITY_PROVIDER_SERVICE_ADMIN_INITIATE_AUTH)
            .header(reqwest::header::CONTENT_TYPE, CONTENT_TYPE)
            .json(&payload)
            .send()?;

        if response.status().is_success() {
            let data: AuthResponse = response.json()?;
            Ok(data.access_token)
        } else {
            Err(MINTError::from_response(response))
        }
    }
}

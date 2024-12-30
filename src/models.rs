use serde::{Deserialize, Serialize};

/// Request payload for SignUp.
#[derive(Serialize, Deserialize)]
pub struct SignUpRequest {
    pub username: String,
    pub password: String,
}
 
/// Request payload for Authenticate.
#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub auth_flow: String,
    pub auth_parameters: AuthParameters,
}

/// Authentication parameters.
#[derive(Serialize, Deserialize)]
pub struct AuthParameters {
    pub username: String,
    pub password: String,
}

/// Response for Authenticate.
#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

use serde::{Deserialize, Serialize};


/// <p>Represents the request to register a user.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SignUpRequest {
    /// <p>The ID of the client associated with the user pool.</p>
    #[serde(rename = "ClientId")]
    pub client_id: String,
    /// <p>The password of the user you wish to register.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The user name of the user you wish to register.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>The response from the server for a registration request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SignUpResponse {
    /// <p>A response from the server indicating that a user registration has been confirmed.</p>
    #[serde(rename = "UserConfirmed")]
    pub user_confirmed: bool,
    /// <p>The UUID of the authenticated user. This is not the same as <code>username</code>.</p>
    #[serde(rename = "UserSub")]
    pub user_sub: String,
}
 
/// Request payload for Authenticate.
#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub auth_flow: String,
    pub auth_parameters: AuthParameters,
    pub client_id: String,
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

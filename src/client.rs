use crate::cognito::CognitoClient;

pub struct MintClient {
    base_url: String,
    client_id: String,
}

impl MintClient {
    pub fn new(base_url: &str, client_id: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client_id: client_id.to_string(),
        }
    }

    pub fn cognito(&self) -> CognitoClient {
        CognitoClient::new(&self.base_url, &self.client_id)
    }
}

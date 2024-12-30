use crate::cognito::CognitoClient;

pub struct MintClient {
    base_url: String,
}

impl MintClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub fn cognito(&self) -> CognitoClient {
        CognitoClient::new(&self.base_url)
    }
}

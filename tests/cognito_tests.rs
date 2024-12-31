#[cfg(test)]
mod tests {
    use mint_rust_sdk::{
        client::MintClient, models::SignUpResponse, utils::random_utility::RandomUtils,
    };

    #[test]
    fn test_auth_success() {
        let client = MintClient::new("http://localhost:3000", "client_id");
        let username = RandomUtils::generate_random_string(10);
        let password = RandomUtils::generate_random_string(10);

        client.cognito().sign_up(&username, &password);

        let result = client.cognito().admin_initiate_auth(&username, &password);

        assert!(result.is_ok());
        let response: mint_rust_sdk::models::AdminInitiateAuthResponse = result.unwrap();
        let authentication_result = response.authentication_result.unwrap();

        assert!(!authentication_result.access_token.is_empty());
        assert!(authentication_result.expires_in > 0);
        assert!(!authentication_result.token_type.is_empty());

        println!("Access Token: {}", authentication_result.access_token);
    }

    #[test]
    fn test_sign_up_success() {
        let client = MintClient::new("http://localhost:3000", "client_id");
        let username = RandomUtils::generate_random_string(10);
        let password = RandomUtils::generate_random_string(10);
        let result = client.cognito().sign_up(&username, &password);

        assert!(result.is_ok());

        let signup_response: SignUpResponse = result.unwrap();
        assert_eq!(signup_response.user_confirmed, true);
    }
}

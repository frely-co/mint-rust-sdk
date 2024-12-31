#[cfg(test)]
mod tests {
    use mint_rust_sdk::{client::MintClient, models::{self, SignUpResponse}, utils::random_utility::RandomUtils};

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

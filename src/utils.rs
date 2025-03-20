use etopay_sdk::types::newtypes::AccessToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
}

/// Fetches an access token for the specified user via keycloak.
/// Required environment variables:
///   - AUTH_URL
///   - AUTH_CLIENT_ID
///   - AUTH_CLIENT_SECRET
pub async fn get_access_token(realm: &str, username: &str, password: &str) -> AccessToken {
    let auth_url = std::env::var("AUTH_URL").unwrap();
    // let realm = std::env::var("KC_REALM").unwrap();
    let url = format!("{auth_url}/realms/{realm}/protocol/openid-connect/token");

    let client_id = std::env::var("AUTH_CLIENT_ID").unwrap();
    let client_secret = std::env::var("AUTH_CLIENT_SECRET").unwrap();

    let params = [
        ("grant_type", "password"),
        ("scope", "openid"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("username", username),
        ("password", password),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .unwrap();

    let response = response.json::<AuthResponse>().await.unwrap();

    AccessToken::try_from(response.access_token).unwrap()
}

use std::path::Path;

use cawaena_sdk::{core::{config::DeserializedConfig, Config, Sdk}, types::{currencies::Currency, newtypes::{EncryptionPin, PlainPassword}}};

mod utils;

const WALLET_PIN: &str = "12345";
const WALLET_PASSWORD: &str = "Strong+Wallet+Pa55word";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    let username = std::env::var("USER_NAME").unwrap();
    let password = std::env::var("USER_PASSWORD").unwrap();
    

    // Replace with the SDK Configuration for your project. Get it from the dashboard: https://dashboard.cawaena.com
    let config_json = r#"
    {
       // Add your SDK configuration here
    }"#;
    // TODO: Merge this
    let config: DeserializedConfig = serde_json::from_str(config_json).expect("failed to parse config");
    let config = Config::try_from(config).expect("failed to create config");

    let realm = config.auth_provider.clone();

    // Initialize SDK from config
    let mut sdk = Sdk::new(config).expect("failed to initialize sdk");
    sdk.set_currency(Currency::Iota);

    // Create new user if user database not exists
    let path_user_db = Path::new("./tmp/sdk-user.db");
    if !path_user_db.exists() {
        sdk.create_new_user(&username).await.unwrap();
    } 

    // Initialize user
    sdk.init_user(&username).await.unwrap();

    // Refresh access token
    let access_token = utils::get_access_token(&realm, &username, &password).await;
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    
    // Set wallet password if not set
    let wallet_pin = EncryptionPin::try_from_string(WALLET_PIN).unwrap();
    let wallet_password = PlainPassword::try_from_string(WALLET_PASSWORD).unwrap();
    if !sdk.is_password_set().await.unwrap() {
        sdk.set_password(&wallet_pin, &wallet_password).await.unwrap();
    }

    // Create new wallet if no wallets exists
    let path_wallets = Path::new("./tmp/wallets");
    if !path_wallets.exists() {
        sdk.create_wallet_from_new_mnemonic(&wallet_pin).await.unwrap();
    }

    // Generate new receiver address
    let address = sdk.generate_new_address(&wallet_pin).await.unwrap();
    println!("Address: {:?}", address);

}

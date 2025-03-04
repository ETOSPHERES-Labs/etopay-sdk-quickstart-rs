
use std::path::Path;

use cawaena_sdk::{core::{config::DeserializedConfig, Config, Sdk}, types::{currencies::Currency, newtypes::{EncryptionPin, PlainPassword}}};

mod utils;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;


const WALLET_PIN: &str = "12345";
const WALLET_PASSWORD: &str = "Strong+Wallet+Pa55word";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let username = std::env::var("USER_NAME").expect("USER_NAME must be set");
    let password = std::env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");
    

    // Replace with the SDK Configuration for your project. Get it from the dashboard: https://dashboard.cawaena.com
    let config_json = r#"
    {
       // Add your SDK configuration here
    }"#;
    // TODO: Merge this
    let config: DeserializedConfig = serde_json::from_str(config_json)?;
    let config = Config::try_from(config)?;

    let realm = config.auth_provider.clone();

    // Initialize SDK from config
    let mut sdk = Sdk::new(config).expect("failed to initialize sdk");
    sdk.set_currency(Currency::Iota);

    // Create new user if user database not exists
    let path_user_db = Path::new("./tmp/sdk-user.db");
    if !path_user_db.exists() {
        sdk.create_new_user(&username).await?;
    }

    // Initialize user
    sdk.init_user(&username).await?;

    // Refresh access token
    let access_token = utils::get_access_token(&realm, &username, &password).await;
    sdk.refresh_access_token(Some(access_token)).await?;

    
    // Set wallet password if not set
    let wallet_pin = EncryptionPin::try_from_string(WALLET_PIN)?;
    let wallet_password = PlainPassword::try_from_string(WALLET_PASSWORD)?;
    if !sdk.is_password_set().await? {
        sdk.set_password(&wallet_pin, &wallet_password).await?;
    }

    // Create new wallet if no wallets exists
    let path_wallets = Path::new("./tmp/wallets");
    if !path_wallets.exists() {
        sdk.create_wallet_from_new_mnemonic(&wallet_pin).await?;
    }

    // Generate new receiver address
    let address = sdk.generate_new_address(&wallet_pin).await?;
    println!("Address: {:?}", address);

    Ok(())

}

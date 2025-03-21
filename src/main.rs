use etopay_sdk::{
    core::{Config, Sdk},
    types::newtypes::{EncryptionPin, PlainPassword},
};
use std::path::Path;
mod utils;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const WALLET_PIN: &str = "12345";
const WALLET_PASSWORD: &str = "Strong+Wallet+Pa55word";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let username = std::env::var("USER_NAME").expect("USER_NAME must be set");
    let password = std::env::var("USER_PASSWORD").expect("USER_PASSWORD must be set");

    // Replace with the SDK Configuration for your project. Get it from the dashboard: https://etopayapp.etospheres.com
    let config = Config::from_json(
        r#"
    
    // Add your SDK configuration here
    
    "#,
    )?;

    let realm = config.auth_provider.clone();

    // Initialize SDK from config
    let mut sdk = Sdk::new(config).expect("failed to initialize sdk");

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

    // Get list of available networks
    let networks = sdk.get_networks().await?;

    // Select which network to use
    sdk.set_network(networks[0].id.clone()).await.unwrap();

    // Set wallet password if not set
    let wallet_pin = EncryptionPin::try_from_string(WALLET_PIN)?;
    let wallet_password = PlainPassword::try_from_string(WALLET_PASSWORD)?;
    if !sdk.is_wallet_password_set().await? {
        sdk.set_wallet_password(&wallet_pin, &wallet_password)
            .await?;
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

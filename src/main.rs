use std::path::Path;

use cawaena_sdk::{
    core::{Config, Sdk},
    types::{
        currencies::Currency,
        newtypes::{EncryptionPin, PlainPassword},
    },
};

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
    let config = Config::from_json(
        r#"
 {
    "auth_provider": "d835ba094905499e86cc7561f9cb3b23",
    "backend_url": "http://localhost:7071/v1",
    "storage_path": "./",
    "log_level": "info"
}    "#,
    )?;

    let realm = config.auth_provider.clone();

    // Initialize SDK from config
    let mut sdk = Sdk::new(config).expect("failed to initialize sdk");

    // Create new user if user database not exists
    // let path_user_db = Path::new("./tmp/sdk-user.db");
    // if !path_user_db.exists() {
    if let Err(e) = sdk.create_new_user(&username).await {
        println!("Error creating user: {e}");
    }

    // Initialize user
    sdk.init_user(&username).await?;

    // Refresh access token
    let access_token = utils::get_access_token(&realm, &username, &password).await;
    sdk.refresh_access_token(Some(access_token)).await?;

    // Set wallet password if not set
    let wallet_pin = EncryptionPin::try_from_string(WALLET_PIN)?;
    let wallet_password = PlainPassword::try_from_string(WALLET_PASSWORD)?;
    if !sdk.is_wallet_password_set().await? {
        sdk.set_wallet_password(&wallet_pin, &wallet_password)
            .await?;
    }

    // Create new wallet if no wallets exists
    // let path_wallets = Path::new("./wallets");
    // if !path_wallets.exists() {
    sdk.create_wallet_from_new_mnemonic(&wallet_pin).await?;
    // }

    println!("share: {:?}", sdk.get_recovery_share().await?);

    sdk.refresh_access_token(None).await?;
    // Generate new receiver address
    sdk.set_currency(Currency::Iota);
    let address = sdk.generate_new_address(&wallet_pin).await?;
    println!("Address: {:?}", address);
    // sdk.set_currency(Currency::Eth);
    // let address = sdk.generate_new_address(&wallet_pin).await?;
    //
    // println!("Address: {:?}", address);

    Ok(())
}

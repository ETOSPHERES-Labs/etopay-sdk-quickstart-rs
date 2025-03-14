
use std::path::Path;
use cawaena_sdk::{core::{Config, Sdk}, types::{networks::{Network, NetworkType}, newtypes::{EncryptionPin, PlainPassword}}};


pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

const WALLET_PIN: &str = "12345";
const WALLET_PASSWORD: &str = "Strong+Wallet+Pa55word";
const WALLET_BACKUP_PASSWORD: &str = "my-backup-pw";
const WALLET_BACKUP_PATH: &str = "./wallet-backup.kdbx";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let username = std::env::var("USER_NAME").expect("USER_NAME must be set");
    let wallet_backup_password = PlainPassword::try_from_string(WALLET_BACKUP_PASSWORD)?;
    let backup_path = Path::new(WALLET_BACKUP_PATH);
    
    // Replace with the SDK Configuration for your project. Get it from the dashboard: https://dashboard.cawaena.com
    let config = Config::from_json(r#"
    
    // Add your SDK configuration here
    
    "#)?;

    // Initialize SDK from config
    let mut sdk = Sdk::new(config).expect("failed to initialize sdk");

    // Create new user and return error if user already exists
    if let Err(e) = sdk.create_new_user(&username).await {
        println!("Error creating user: {e}");
    }

    // Initialize user
    sdk.init_user(&username).await?;

    // Set list of available networks
    sdk.set_networks(vec![
        Network {
            id: String::from("67a1f08edf55756bae21e7eb"),
            name: String::from("IOTA"),
            currency: String::from("IOTA"),
            block_explorer_url: String::from("https://explorer.shimmer.network/testnet/"),
            enabled: true,
            network_identifier: Some(String::from("iota_mainnet")),
            network_type: NetworkType::Stardust {
                node_url: String::from("https://api.testnet.iotaledger.net"),
            },
        }
    ]);
    
    // Select which network to use
    sdk.set_network(String::from("67a1f08edf55756bae21e7eb")).await.unwrap();
    
    // Set wallet password if not set
    let wallet_pin = EncryptionPin::try_from_string(WALLET_PIN)?;
    let wallet_password = PlainPassword::try_from_string(WALLET_PASSWORD)?;
    if !sdk.is_wallet_password_set().await? {
        sdk.set_wallet_password(&wallet_pin, &wallet_password).await?;
    }

    // Create new wallet if no backup exists else restore from backup
    if !backup_path.exists() {
        let mnemonic = sdk.create_wallet_from_new_mnemonic(&wallet_pin).await?;
        println!("Mnemonic: {:?}", mnemonic);
    } else {
        // Restore wallet from backup
        let backup = std::fs::read(backup_path)?;
        sdk.create_wallet_from_backup(&wallet_pin, &backup, &wallet_backup_password).await?;
    }

    // Create wallet backup
    let backup = sdk.create_wallet_backup(&wallet_pin, &wallet_backup_password).await?;
    // Store as file
    std::fs::write(backup_path, backup)?;
    println!("Wallet backup stored at: {:?}", backup_path);

    // Generate new receiver address
    let address = sdk.generate_new_address(&wallet_pin).await?;
    println!("Address: {:?}", address);

    Ok(())

}

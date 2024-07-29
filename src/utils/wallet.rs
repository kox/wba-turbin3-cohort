use std::{collections::HashMap, error::Error};

use rocksdb::DB;
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Wallet {
    pub pubkey: String,
    pub secret_key: Vec<u8>,
}

pub fn get_wallet_key(name: &str) -> String {
    format!("wallet:{}", name)
}

pub fn save_wallet_to_db(wallet: &Wallet, db: &DB, wallet_name: &str) -> Result<(), Box<dyn Error>> {
    // Serialize the Wallet struct
    let wallet_value = serde_json::to_vec(wallet)?;

    // Print debug information
    println!("Saving wallet to DB with key: {}", get_wallet_key(wallet_name));
    println!("Wallet value: {:?}", wallet);

    // Save the Wallet struct to RocksDB
    db.put(get_wallet_key(wallet_name), wallet_value)?;

    Ok(())
}

pub fn generate_keypair(db: &DB, name: &str) {
    let keypair = Keypair::new();

    println!(
        "You've generated a new Solana wallet: {}",
        keypair.try_pubkey().unwrap().to_string()
    );

    let wallet = Wallet {
        pubkey: keypair.pubkey().to_string(),
        secret_key: keypair.to_bytes().to_vec(),
    };

    save_wallet_to_db(&wallet, db, name).expect("Failed to save wallet to RocksDB");
    println!("Wallet {} saved to RocksDB", name);
}

pub fn read_wallet(db: &DB, name: &str) -> Wallet {
    let wallet_key = get_wallet_key(name);
    println!("Reading wallet from DB with key: {}", wallet_key); // Debug print

    let wallet_json = db
        .get(&wallet_key)
        .expect("Failed to read from RocksDB")
        .expect("Wallet not found in RocksDB");

    let wallet: Wallet = serde_json::from_slice(&wallet_json).expect("Failed to deserialize wallet");

    println!("You've read a Solana wallet: {} from db", wallet.pubkey);

    wallet
}

pub fn list_wallets(db: &DB) -> HashMap<String, Wallet> {
    let mut wallets = HashMap::new();
    let prefix = b"wallet:";

    for item in db.prefix_iterator(prefix) {
        match item {
            Ok((key, value)) => {
                let key_str = String::from_utf8(key.to_vec()).unwrap();
                let wallet: Wallet = serde_json::from_slice(&value).unwrap();
                wallets.insert(key_str, wallet);
            }
            Err(e) => {
                eprintln!("Error reading from RocksDB: {:?}", e);
            }
        }
    }

    wallets
}


pub fn base58_to_wallet(pkey: &str) -> Result<Wallet, Box<dyn Error>> {
    // Decode the base58 string into a byte array
    let decoded_secret_key = bs58::decode(pkey).into_vec()?;

    // Create a Keypair from the secret key
    let keypair = Keypair::from_bytes(&decoded_secret_key)?;

    // Get the public key from the keypair
    let public_key = keypair.pubkey();

    // Create the Wallet struct
    let wallet = Wallet {
        pubkey: public_key.to_string(),
        secret_key: decoded_secret_key,
    };

    Ok(wallet)
}

pub fn wallet_to_base58(wallet: Wallet) -> String {
    // Convert the secret key to base58
    let base58 = bs58::encode(wallet.secret_key).into_string();

    println!("Your private key in base58 is: {:?}", base58);

    base58
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_generate_and_read_wallet() {
        let tmp_dir = TempDir::new("test_db").unwrap();
        let db = DB::open_default(tmp_dir.path()).unwrap();

        let wallet_name = "test_wallet";
        generate_keypair(&db, wallet_name);

        let wallet = read_wallet(&db, wallet_name);

        assert_eq!(wallet.pubkey.len(), 44); // Length of a base58-encoded Solana pubkey
        assert_eq!(wallet.secret_key.len(), 64); // Length of a Solana secret key
    }

    #[test]
    fn test_list_wallets() {
        let tmp_dir = TempDir::new("test_db").unwrap();
        let db = DB::open_default(tmp_dir.path()).unwrap();

        let wallet1_name = "wallet1";
        let wallet2_name = "wallet2";

        generate_keypair(&db, wallet1_name);
        generate_keypair(&db, wallet2_name);

        let wallets = list_wallets(&db);

        assert_eq!(wallets.len(), 2);
        assert!(wallets.contains_key(&get_wallet_key(wallet1_name)));
        assert!(wallets.contains_key(&get_wallet_key(wallet2_name)));
    }

    #[test]
    fn test_base58_to_wallet() {
        // Generate a keypair for testing
        let keypair = Keypair::new();

        // Encode the secret key to base58
        let base58_secret_key = bs58::encode(keypair.to_bytes()).into_string();

        // Call the function
        let result = base58_to_wallet(&base58_secret_key);

        // Check the result
        assert!(result.is_ok());
        let wallet = result.unwrap();

        // Verify the public key
        assert_eq!(wallet.pubkey, keypair.pubkey().to_string());

        // Verify the secret key
        assert_eq!(wallet.secret_key, keypair.to_bytes().to_vec());
    }

    #[test]
    fn test_wallet_to_base58() {
        let tmp_dir = TempDir::new("test_db").unwrap();
        let db = DB::open_default(tmp_dir.path()).unwrap();

        let wallet_name = "test_wallet";
        generate_keypair(&db, wallet_name);

        let wallet = read_wallet(&db, wallet_name);

        // Expected Base58 encoded string for the given secret_key
        let expected_base58 = bs58::encode(&wallet.secret_key).into_string();

        // Call the wallet_to_base58 function
        let base58_result = wallet_to_base58(wallet);

        // Assert the result matches the expected Base58 string
        assert_eq!(base58_result, expected_base58);
    }
}

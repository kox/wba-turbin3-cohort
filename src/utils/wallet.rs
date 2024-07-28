use std::collections::HashMap;

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

    let wallet_json = serde_json::to_string_pretty(&wallet).unwrap();
    db.put(get_wallet_key(name), wallet_json).unwrap();

    println!("Wallet {} saved to RocksDB", name);
}

pub fn read_wallet(db: &DB, name: &str) -> Wallet {
    let wallet_json = db.get(get_wallet_key(name)).unwrap().unwrap();

    let wallet: Wallet = serde_json::from_slice(&wallet_json).unwrap();

    println!(
        "You've read a Solana wallet: {} from db",
        wallet.pubkey
    );

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
}

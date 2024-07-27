use rocksdb::DB;
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub pubkey: String,
    pub secret_key: Vec<u8>,
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
    db.put(name, wallet_json).unwrap();

    println!("Wallet {} saved to RocksDB", name);
}

pub fn read_wallet(db: &DB, name: &str) {
    let wallet_json = db.get(name).unwrap().unwrap();

    let wallet: Wallet = serde_json::from_slice(&wallet_json).unwrap();

    println!(
        "You've read a new Solana wallet: {}",
        wallet.pubkey.to_string()
    );
}
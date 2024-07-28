mod utils; 
use rocksdb::DB;
use clap::{Parser, Subcommand};
use solana_sdk::signature::Keypair;

#[derive(Subcommand)]
enum Commands {
    Keygen {
        name: String,
    },
    ReadKeygen {
        name: String,
    },
    ListWallets,
    Base58ToWallet {
        pkey_bs58: String,
        name: String,
    },
    /* 
    WalletToBase58,
    Airdrop,
    Transfer {
        to: String,
        amount: u64,
    },
    CleanWallet,
    Submit, */
}

#[derive(Parser)]
#[command(name = "WBA Turbin3 Toolkit CLI")]
#[command(version = "1.0")]
#[command(author = "kox <garsanzi@gmail.com>")]
#[command(about = "Does awesome things with Solana")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = "wba_toolkit", global = true)]
    db_path: String,
}

fn main() {
    println!("welcome to main");

    let cli = Cli::parse();
    let db = DB::open_default(&cli.db_path).unwrap();

    match cli.command {
        Commands::Keygen {name } => utils::wallet::generate_keypair(&db, &name),
        Commands::ReadKeygen {name } => {
            let wallet = utils::wallet::read_wallet(&db, &name);
            println!("Wallet read: {:?}", wallet.pubkey);
        },
        Commands::ListWallets => {
            let wallets = utils::wallet::list_wallets(&db);
            println!("wallets: {:?}", wallets)
        },
        Commands::Base58ToWallet { pkey_bs58, name } => {
            match utils::wallet::base58_to_wallet(&pkey_bs58) {
                Ok(wallet) => {
                    utils::wallet::save_wallet_to_db(&wallet, &db, &name).unwrap();
                    println!("Wallet created and saved successfully!");
                },
                Err(e) => {
                    eprintln!("Error creating wallet: {}", e);
                },
            }
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use solana_sdk::signer::Signer;
    use tempdir::TempDir;
    use utils::wallet::read_wallet;

    #[test]
    fn test_keygen_command() {
        let tmp_dir = TempDir::new("wallet_db").unwrap();
        let db_path = tmp_dir.path().to_str().unwrap();

        Command::cargo_bin("turbin3_pre_req")
            .unwrap()
            .arg("keygen")
            .arg("test_wallet")
            .arg("--db-path")
            .arg(db_path)
            .assert()
            .success();
            // .stdout(contains("Generated wallet"));

        // Verify the wallet was stored in RocksDB
        let db = DB::open_default(db_path).unwrap();
        let wallet = read_wallet(&db, "test_wallet");
        assert_eq!(wallet.pubkey.len(), 44); // Check that the pubkey is of correct length
        assert_eq!(wallet.secret_key.len(), 64); // Check that the secret_key is of correct length
    }

    #[test]
    fn test_list_wallets() {
        let tmp_dir = TempDir::new("wallet_db").unwrap();
        let db_path = tmp_dir.path().to_str().unwrap();

        // Create multiple wallets
        Command::cargo_bin("turbin3_pre_req")
            .unwrap()
            .arg("keygen")
            .arg("wallet1")
            .arg("--db-path")
            .arg(db_path)
            .assert()
            .success();

        Command::cargo_bin("turbin3_pre_req")
            .unwrap()
            .arg("keygen")
            .arg("wallet2")
            .arg("--db-path")
            .arg(db_path)
            .assert()
            .success();

        // List wallets
        Command::cargo_bin("turbin3_pre_req")
            .unwrap()
            .arg("list-wallets")
            .arg("--db-path")
            .arg(db_path)
            .assert()
            .success()
            .stdout(predicates::str::contains("wallet:wallet1"))
            .stdout(predicates::str::contains("wallet:wallet2"));
    }

    #[test]
    fn test_base58_to_wallet_command() {
        let tmp_dir = TempDir::new("wallet_db").unwrap();
        let db_path = tmp_dir.path().to_str().unwrap();

        // Generate a keypair for testing
        let keypair = Keypair::new();
        let base58_secret_key = bs58::encode(keypair.to_bytes()).into_string();

        // Run the command
        Command::cargo_bin("turbin3_pre_req")
            .unwrap()
            .arg("base58-to-wallet")
            .arg(&base58_secret_key)
            .arg("test_wallet")
            .arg("--db-path")
            .arg(db_path)
            .assert()
            .success()
            .stdout(predicates::str::contains("Wallet created and saved successfully!"));

        // Verify the wallet was stored in RocksDB
        let db = DB::open_default(db_path).unwrap();
        let wallet = read_wallet(&db, "test_wallet");
        assert_eq!(wallet.pubkey, keypair.try_pubkey().unwrap().to_string());
        assert_eq!(wallet.secret_key, keypair.to_bytes().to_vec());
    }
}

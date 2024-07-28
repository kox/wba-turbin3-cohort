/* mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::wba_prereq::{ WbaPrereqProgram, CompleteArgs };
    use serde::{ Deserialize, Serialize };
    use serde_json::from_reader;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        system_program,
        system_instruction::transfer,
    };
    use solana_sdk::{
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    use std::fs::File;
    use std::io::{ BufReader, Write };
    use std::str::FromStr;

    #[derive(Serialize, Deserialize)]
    struct Wallet {
        pubkey: String,
        secret_key: Vec<u8>,
    }

    #[test]
    fn keygen() {
        // Create a new keypair
        let keypair = Keypair::new();

        println!(
            "You've generated a new Solana wallet: {}",
            keypair.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", keypair.to_bytes());

        // Create the Wallet struct
        let wallet = Wallet {
            pubkey: keypair.pubkey().to_string(),
            secret_key: keypair.to_bytes().to_vec(),
        };

        // Serialize the Wallet struct to a JSON string
        let wallet_json = serde_json::to_string_pretty(&wallet).unwrap();

        // Write the JSON string to a file
        let mut file = File::create("dev-wallet.json").unwrap();
        file.write_all(wallet_json.as_bytes()).unwrap();

        println!("Wallet saved to dev-wallet.json");
    }

    #[test]
    fn base58_to_wallet() {
        /*  println!("Input your private key as base58:");
        let stdin = stdin(); */
        println!("Reading the wallet file...");

        // Open the JSON file
        let file = File::open("dev-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        // Convert the secret key to base58
        let base58 = bs58::encode(wallet.secret_key.clone()).into_string();
        println!("Your private key in base58 is: {:?}", base58);

        // Convert the base58 back to wallet byte array
        let decoded_wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Decoded wallet byte array is: {:?}", decoded_wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Reading the wallet file...");

        // Open the JSON file
        let file = File::open("dev-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        // Convert the secret key to base58
        let base58 = bs58::encode(wallet.secret_key).into_string();
        println!("Your private key in base58 is: {:?}", base58);
    }

    #[test]
    fn airdop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Open the JSON file
        let file = File::open("dev-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        let keypair = Keypair::from_bytes(&wallet.secret_key).expect("Invalid keypair");

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Open the JSON file
        let file = File::open("dev-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        let keypair = Keypair::from_bytes(&wallet.secret_key).expect("Invalid keypair");

        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("Be9MdYwSsMUTLCA3pV9FaVsPDSJyuokjeNZLoaU13s1W").unwrap();

        // Get recent blockhash
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Let's transfer 0.1 SOL
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn clean_wallet() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Open the JSON file
        let file = File::open("dev-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        let keypair = Keypair::from_bytes(&wallet.secret_key).expect("Invalid keypair");

        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("Be9MdYwSsMUTLCA3pV9FaVsPDSJyuokjeNZLoaU13s1W").unwrap();

        // Let's check the balance
        let balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        // Print our transaction out
        println!(
            "Left {} lamports!",
            balance
        );

        // Get recent blockhash
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        // Print our transaction out
        println!(
            "Expected Fee: {} !",
            fee
        );

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        // Send the transaction
        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn submit() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Open the JSON file
        let file = File::open("wba-wallet.json").expect("Could not open file");
        let reader = BufReader::new(file);

        // Deserialize the JSON to a Wallet struct
        let wallet: Wallet = from_reader(reader).expect("Could not read JSON");

        let signer = Keypair::from_bytes(&wallet.secret_key).expect("Invalid keypair");
        
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq", 
            signer.pubkey().to_bytes().as_ref()
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"kox".to_vec()
        };

        // Get recent blockhash
        let blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        // Send the transaction
        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);  
    }
}
 */
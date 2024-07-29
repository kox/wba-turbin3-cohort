use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer,
    system_instruction::transfer, transaction::Transaction,
};

use super::wallet::Wallet;

pub fn airdop(wallet: Wallet, rpc_url: &str) {
    // Connected to Solana Devnet RPC Client
    let client = RpcClient::new(rpc_url);

    // We will create a keypair from our wallet struct
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

pub fn transfer_sol(sender: Wallet, receiver: &str, amount: u64, cluster_url: &str) {
    // Connected to Solana Devnet RPC Client
    let client = RpcClient::new(cluster_url);

    let sender_keypair = Keypair::from_bytes(&sender.secret_key)
        .expect("Failed to create a Keypair from the sender Wallet");
    let receiver_pubkey =
        Pubkey::from_str(receiver).expect("Failed to create Pubkey from receiver Wallet");

    // Get recent blockhash
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Let's transfer 0.1 SOL
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&sender_keypair.pubkey(), &receiver_pubkey, amount)], // 100_000_000
        Some(&sender_keypair.pubkey()),
        &vec![&sender_keypair],
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

pub fn clean_wallet(from: Wallet, to: &str, cluster_url: &str) {
    // Connected to Solana Devnet RPC Client
    let client = RpcClient::new(cluster_url);

    // Deserialize the JSON to a Wallet struct
    let keypair = Keypair::from_bytes(&from.secret_key)
        .expect("Failed to create a Keypair from the sender Wallet");

    // Define our WBA public key
    let to_pubkey = Pubkey::from_str(to).expect("Failed to create Pubkey from receiver Wallet ");

    // Let's check the balance
    let balance = client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
    // Print our transaction out
    println!("Left {} lamports!", balance);

    // Get recent blockhash
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    // Print our transaction out
    println!("Expected Fee: {} !", fee);

    // Deduct fee from lamports amount and create a TX with correct balance
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
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

/*
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
    } */

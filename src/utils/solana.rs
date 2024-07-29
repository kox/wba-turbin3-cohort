use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction};

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

        let sender_keypair = Keypair::from_bytes(&sender.secret_key).expect("Failed to create a Keypair from the sender Wallet");
        let receiver_pubkey = Pubkey::from_str(receiver).expect("Failed to create Pubkey from receiver Wallet");
        
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
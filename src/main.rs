mod utils; 
use rocksdb::DB;
use crate::utils::wallet::Wallet;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
    Keygen {
        name: String,
    },
    ReadKeygen {
        name: String,
    },
    /* Base58ToWallet,
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
}

fn main() {
    println!("welcome to main");

    let cli = Cli::parse();
    let db = DB::open_default("wba_toolkit").unwrap();

    match cli.command {
        Commands::Keygen {name } => utils::wallet::generate_keypair(&db, &name),
        Commands::ReadKeygen {name } => utils::wallet::read_wallet(&db, &name),
    }
/* 
    let matches = App::new("Solana CLI")
        .version("1.0")
        .author("Your Name <youremail@example.com>")
        .about("Does awesome things with Solana")
        .subcommand(SubCommand::with_name("keygen").about("Generate a new keypair"))
        .get_matches();
 */
    /* let wallet = Wallet {
        pubkey: String::from("test"),
        secret_key: vec![20, 30]
    };

    print!("testthis {} -> ", wallet.pubkey); */
/* 
    let matches = App::new("Wba Turbin3 Cohort")
        .version("1.0")
        .author("kox <garsanzi@gmail.com>")
        .about("Toolkit to ")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .takes_value(true),
        )
        .get_matches(); */
}

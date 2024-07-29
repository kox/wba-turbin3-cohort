# WBA Turbin3 Toolkit CLI for 7th Builders Cohort Edition

## Overview

WBA Turbin3 Toolkit CLI is a command-line interface tool for managing Solana wallets and different Solana ops. 

This tool allows you to generate new wallets, read existing wallets, list all wallets, and perform various other wallet-related operations.

## Features

- **Generate Keypair**: Create a new Solana wallet.
- **Read Keypair**: Retrieve details of an existing wallet.
- **List Wallets**: Display all wallets stored in the database.
- **Base58 to Wallet**: Create a wallet from a Base58 encoded private key.
- **Wallet to Base58**: Convert a wallet's private key to Base58.
- **Airdrop**: Request an airdrop of SOL tokens.
- **Transfer**: Transfer SOL tokens between wallets.

## Installation

To install the WBA Turbin3 Toolkit CLI, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/kox/wba_turbin3_toolkit
cd wba_turbin3_toolkit
cargo build --release
```


## Usage 

### Generate Keypair

Generates a new Solana wallet and stores it in the RocksDB database.

```bash

./target/release/turbin3_pre_req keygen --db-path <path_to_db> <wallet_name>
```

Arguments:

    <wallet_name>: The name of the wallet to generate.

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.

### Read Keypair

Retrieves and displays the details of an existing wallet.

```bash

./target/release/turbin3_pre_req readkeygen --db-path <path_to_db> <wallet_name>
```

Arguments:

    <wallet_name>: The name of the wallet to read.

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.

### List Wallets

Lists all the wallets stored in the database.

```bash

./target/release/turbin3_pre_req list-wallets --db-path <path_to_db>
```

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.

### Base58 to Wallet
Creates a wallet from a Base58 encoded private key and stores it in the RocksDB database.

```bash
./target/release/turbin3_pre_req base58-to-wallet <pkey_bs58> <wallet_name> --db-path <path_to_db>
```

Arguments:

    <pkey_bs58>: The Base58 encoded private key.
    <wallet_name>: The name of the wallet to create.

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.

### Wallet to Base58
Converts a wallet's private key to Base58 and prints it.

```bash
./target/release/turbin3_pre_req wallet-to-base58 <wallet_name> --db-path <path_to_db>
```

Arguments:

    <wallet_name>: The name of the wallet.

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.

### Airdrop

Requests an airdrop of 2 SOL tokens to the specified wallet.

```bash
./target/release/turbin3_pre_req airdrop <wallet_name> --db-path <path_to_db> --cluster-url <cluster_url>
```

Arguments:

    <wallet_name>: The name of the wallet.

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.
    --cluster-url <cluster_url>: The URL of the Solana cluster. Defaults to https://api.devnet.solana.com.

### Transfer
Transfers SOL tokens between wallets.

```bash
    ./target/release/turbin3_pre_req transfer <from_wallet> <to_wallet> <amount> --db-path <path_to_db> --cluster-url <cluster_url>
```

Arguments:

    <from_wallet>: The name of the sender's wallet.
    <to_wallet>: The public key of the recipient's wallet.
    <amount>: The amount of SOL to transfer (in lamports, 1 SOL = 1,000,000,000 lamports).

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.
    --cluster-url <cluster_url>: The URL of the Solana cluster. Defaults to https://api.devnet.solana.com.

### Clean Wallet
Transfers the remaing SOL tokens from a Wallet to another wallet. After that the from wallet account gets closed.

```bash
    ./target/release/turbin3_pre_req clean-wallet <from_wallet> <to_wallet>  --db-path <path_to_db> --cluster-url <cluster_url>
```

Arguments:

    <from_wallet>: The name of the sender's wallet.
    <to_wallet>: The public key of the recipient's wallet.
    

Options:

    --db-path <path_to_db>: The path to the RocksDB database. Defaults to wba_toolkit.
    --cluster-url <cluster_url>: The URL of the Solana cluster. Defaults to https://api.devnet.solana.com.

## Example

Here is an example workflow:

### Generate a new wallet:

```bash

./target/release/turbin3_pre_req keygen --db-path my_wallets_db wallet1
```

### Read an existing wallet:

```bash

./target/release/turbin3_pre_req readkeygen --db-path my_wallets_db wallet1
```

### List all wallets:

```bash

./target/release/turbin3_pre_req list-wallets --db-path my_wallets_db
```

### Create a wallet from a Base58 private key:

    ./target/release/turbin3_pre_req base58-to-wallet <base58_private_key> wallet1 --db-path my_wallets_db

### Convert a wallet's private key to Base58:

    ./target/release/turbin3_pre_req wallet-to-base58 wallet1 --db-path my_wallets_db
### Request an airdrop of 2 SOL tokens:

    ./target/release/turbin3_pre_req airdrop wallet1 --db-path my_wallets_db --cluster-url https://api.devnet.solana.com

### Transfer 1 SOL from one wallet to another:
    ./target/release/turbin3_pre_req transfer wallet1 Be9MdYwSsMUTLCA3pV9FaVsPDSJyuokjeNZLoaU13s1W 1000000000 --db-path my_wallets_db --cluster-url https://api.devnet.solana.com

### Clean Wallet from one wallet to another:
    ./target/release/turbin3_pre_req clean-wallet wallet1 Be9MdYwSsMUTLCA3pV9FaVsPDSJyuokjeNZLoaU13s1W --db-path my_wallets_db --cluster-url https://api.devnet.solana.com


## Running Tests

To run the tests for the CLI application, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with any improvements or bug fixes.

## License

This project is licensed under the MIT License.

## Explanation:

1. **Installation**: Instructions on how to clone and build the project.
2. **Usage**: Detailed instructions for each command with examples.
3. **Examples**: Workflow examples demonstrating how to use the CLI.
4. **Testing**: Instructions on running tests.
5. **Contributing**: Information for potential contributors.
6. **License**: The project's license information.


## PreReq
I applied some changes to directly read and save the dev wallet generated in the first step. Also to read from the file mine.

Check lib.rs to run all tests. Probably you will need to change a bit of stuff to make it work on your local env.


### Proof of Work 

https://explorer.solana.com/tx/4hty78jUkZEiSrX2tSx5XU93L289nerZr7eYqc2tbFaSPWbULQVRpRz5CizwxPUp7iAdeGprnQbAZjuFFZRhvMVU?cluster=devnet


###  Cheers



"wba_toolkit"
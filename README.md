# WBA Turbin3 Toolkit CLI for 7th Builders Cohort Edition

## Overview

WBA Turbin3 Toolkit CLI is a command-line interface tool for managing Solana wallets and different Solana ops. 

This tool allows you to generate new wallets, read existing wallets, list all wallets, and perform various other wallet-related operations.

## Features

- **Generate Keypair**: Create a new Solana wallet.
- **Read Keypair**: Retrieve details of an existing wallet.
- **List Wallets**: Display all wallets stored in the database.

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

## Running Tests

To run the tests for the CLI application, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with any improvements or bug fixes.

## License

This project is licensed under the MIT License.


## PreReq
I applied some changes to directly read and save the dev wallet generated in the first step. Also to read from the file mine.

Check lib.rs to run all tests. Probably you will need to change a bit of stuff to make it work on your local env.


### Proof of Work 

https://explorer.solana.com/tx/4hty78jUkZEiSrX2tSx5XU93L289nerZr7eYqc2tbFaSPWbULQVRpRz5CizwxPUp7iAdeGprnQbAZjuFFZRhvMVU?cluster=devnet


###  Cheers



"wba_toolkit"
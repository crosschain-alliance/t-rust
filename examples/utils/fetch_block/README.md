# Fetch Block

This command-line utility fetches an Ethereum block from a specified node RPC URL, validates it against Mainnet consensus rules, and saves the serialized block to a file.

## Features

- **Fetch Block**: Retrieve a specific block or the latest block from an Ethereum node.
- **Validate Block**: Validates the fetched block using pre-execution consensus rules.
- **Serialize Block**: Encodes the block into binary format and writes it to an output file.

## Prerequisites

- [Rust](https://www.rust-lang.org/) and Cargo installed
- An Ethereum node RPC endpoint (e.g., Infura, Alchemy, or a locally hosted node)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd <repository_directory>/sdk/examples/utils/fetch_block
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the compiled binary with the appropriate command line options:

```bash
cargo run --release -- --rpc_url <RPC_URL> [--block_number <BLOCK_NUMBER>] [--output_file <OUTPUT_FILE>]
```

### Command Line Arguments

- `--rpc_url RPC_URL` (required):  
  The Ethereum node RPC URL to fetch the block from.  
  _Example_: `--rpc_url https://mainnet.infura.io/v3/YOUR_PROJECT_ID`

- `-n, --block_number BLOCK_NUMBER` (optional):  
  The block number to fetch. If omitted, the utility will fetch the latest block available.  
  _Example_: `--block_number 12345678`

- `-o, --output_file OUTPUT_FILE` (optional):  
  The file path where the fetched and serialized block will be saved.  
  Defaults to `reth_block.bin` if not provided.  
  _Example_: `--output_file my_block.bin`

### Example

To fetch block number 12345678 and save it to `my_block.bin`:

```bash
cargo run --release -- --rpc_url https://mainnet.infura.io/v3/YOUR_PROJECT_ID --block_number 12345678 --output_file my_block.bin
```

If no block number is provided, the utility will use the latest block:

```bash
cargo run --release -- --rpc_url https://mainnet.infura.io/v3/YOUR_PROJECT_ID
```

use alloy::network::{Ethereum, Network};
use alloy::network::primitives::BlockTransactionsKind;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::serde_helpers::WithOtherFields;
use alloy_rlp::{Encodable};
use alloy::transports::{
    http::{Client, Http},
    layers::{RetryBackoffLayer, RetryBackoffService},
};
use eyre::Result;
use reth_consensus_common::validation::validate_block_pre_execution;
use reth_primitives::{
    Block, BlockBody, Header, TransactionSigned, Withdrawals,
};
use std::fs::File;
use std::io::{Write};
use reth_chainspec::MAINNET;
use clap::Parser;
use url::Url;

// New command line options using clap.
#[derive(Parser, Debug)]
#[clap(author, version, about = "Fetch and validate an Ethereum block", long_about = None)]
struct Args {
    /// The RPC URL to fetch the block from.
    #[clap(long, value_name = "RPC_URL", help = "Provide the Ethereum node RPC URL.")]
    rpc_url: String,

    /// The block number to fetch. If not provided, fetches the latest block.
    #[clap(short = 'n', long, value_name = "BLOCK_NUMBER", help = "Specify the block number to fetch.")]
    block_number: Option<u64>,

    /// The file path where the fetched block will be saved.
    #[clap(short = 'o', long, value_name = "OUTPUT_FILE", default_value = "reth_block.bin", help = "Output file to save the fetched block.")]
    output_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let rpc_url = args.rpc_url;

    let retry_layer = RetryBackoffLayer::new(10, 100, 330);
    let client = RpcClient::builder()
            .layer(retry_layer)
            .http(Url::parse(&rpc_url)?);
    let http_client: RootProvider<RetryBackoffService<Http<Client>>, Ethereum> = ProviderBuilder::new().network().on_client(client);

    let block_number: u64 = match args.block_number {
        Some(n) => n,
        None => http_client.get_block_number().await?,
    };

    println!("Block number: {}", block_number);

    // Fetch the full block details
    let alloy_block = http_client.get_block(block_number.into(), BlockTransactionsKind::Full).await?;

    let reth_block = derive_block(alloy_block.unwrap());

    let sealed_block = reth_block.clone().seal_slow();
    match validate_block_pre_execution(&sealed_block, &MAINNET.clone()) {
        Ok(_) => println!("Block is valid!"),
        Err(e) => println!("Block validation failed: {}", e),
    }

    // Save the block to a file
    let mut file = File::create(&args.output_file)?;
    let mut encoded_block = Vec::new();
    reth_block.encode(&mut encoded_block);
    file.write_all(&encoded_block)?;

    println!("Block saved to {}", args.output_file);

    Ok(())
}

fn derive_transaction(transaction: <Ethereum as Network>::TransactionResponse) -> TransactionSigned {
    TransactionSigned::try_from(WithOtherFields::new(transaction)).unwrap()
}

fn derive_block(
    block: <Ethereum as Network>::BlockResponse,
) -> Block {

    Block {
        header: Header::try_from(block.header.clone()).unwrap(),
        body: BlockBody {
            transactions: block
                .transactions
                .into_transactions()
                .map(derive_transaction)
                .collect(),
            ommers: vec![],
            withdrawals: block.withdrawals.map(Withdrawals::new),
            requests: None,
        },
    }
}



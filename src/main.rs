use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::{BlockId, BlockTransactionsKind, BlockNumberOrTag};
use alloy::consensus::Transaction;
use anyhow::Result;
use heimdall_decompiler::{decompile, DecompilerArgsBuilder};

use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // setup our http and websocket provider
    let ws_url = "wss://eth.merkle.io".to_string();
    let http_url = "https://eth.merkle.io".to_string();
    let websocket = WsConnect::new(ws_url);
    let ws_provider = ProviderBuilder::new().on_ws(websocket).await?;
    let http_provider = ProviderBuilder::new().on_http(http_url.parse()?);

    // subscribe to a block stream
    let sub = ws_provider.subscribe_blocks().await?;
    let mut stream = sub.into_stream();
    while stream.next().await.is_some() {
        // get the block and the transactions
        let rpc_block = http_provider.get_block(BlockId::Number(BlockNumberOrTag::Latest), BlockTransactionsKind::Full).await?.unwrap();
        let transactions = rpc_block.transactions.into_transactions();

        for tx in transactions {
            // if the to is none, then this is a contract deployment that we want to decompile
            if tx.to().is_none() {
                let from = tx.from;
                let contract_code = tx.input();

                 let args = DecompilerArgsBuilder::new()
                    .target(contract_code.to_string())
                    .rpc_url(http_url.clone())
                    .include_solidity(true)
                    .build()?;

                let result = decompile(args).await?;
                println!("From: {}, {}", from, result.source.unwrap());

            }
        }
    }

    Ok(())


}


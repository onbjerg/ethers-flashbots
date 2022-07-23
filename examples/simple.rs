use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::convert::TryFrom;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the network
    let provider = Provider::<Http>::try_from("https://mainnet.eth.aragon.network")?;

    // This is your searcher identity
    let bundle_signer = LocalWallet::new(&mut thread_rng());
    // This signs transactions
    let wallet = LocalWallet::new(&mut thread_rng());

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // Pay Vitalik using a Flashbots bundle!
    let tx = TransactionRequest::pay("vitalik.eth", 100);
    let pending_tx = client.send_transaction(tx, None).await?;

    // Get the receipt
    let receipt = pending_tx
        .await?
        .ok_or_else(|| eyre::eyre!("tx not included"))?;
    let tx = client.get_transaction(receipt.transaction_hash).await?;

    println!("Sent transaction: {}\n", serde_json::to_string(&tx)?);
    println!("Receipt: {}\n", serde_json::to_string(&receipt)?);

    Ok(())
}

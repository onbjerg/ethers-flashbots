use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::convert::TryFrom;
use url::Url;

// See https://www.mev.to/builders for up to date builder URLs
static BUILDER_URLS: &[&str] = &[
    "https://builder0x69.io",
    "https://rpc.beaverbuild.org",
    "https://relay.flashbots.net",
    "https://rsync-builder.xyz",
    "https://rpc.titanbuilder.xyz",
    "https://api.blocknative.com/v1/auction",
    "https://mev.api.blxrbdn.com",
    "https://eth-builder.com",
    "https://builder.gmbit.co/rpc",
    "https://buildai.net",
    "https://rpc.payload.de",
    "https://rpc.lightspeedbuilder.info",
    "https://rpc.nfactorial.xyz",
    "https://rpc.lokibuilder.xyz",
];

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
        BroadcasterMiddleware::new(
            provider,
            BUILDER_URLS
                .iter()
                .map(|url| Url::parse(url).unwrap())
                .collect(),
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // get last block number
    let block_number = client.get_block_number().await?;

    // Build a custom bundle that pays 0x0000000000000000000000000000000000000000
    let tx = {
        let mut inner: TypedTransaction = TransactionRequest::pay(Address::zero(), 100).into();
        client.fill_transaction(&mut inner, None).await?;
        inner
    };
    let signature = client.signer().sign_transaction(&tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(tx.rlp_signed(&signature))
        .set_block(block_number + 1)
        .set_simulation_block(block_number)
        .set_simulation_timestamp(0);

    // Send it
    let results = client.inner().send_bundle(&bundle).await?;

    // You can also optionally wait to see if the bundle was included
    for result in results {
        match result {
            Ok(pending_bundle) => match pending_bundle.await {
                Ok(bundle_hash) => println!(
                    "Bundle with hash {:?} was included in target block",
                    bundle_hash
                ),
                Err(PendingBundleError::BundleNotIncluded) => {
                    println!("Bundle was not included in target block.")
                }
                Err(e) => println!("An error occured: {}", e),
            },
            Err(e) => println!("An error occured: {}", e),
        }
    }

    Ok(())
}

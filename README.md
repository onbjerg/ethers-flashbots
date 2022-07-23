## Ethers Flashbots

[![CI Status][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]

[ci-badge]: https://github.com/onbjerg/ethers-flashbots/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/onbjerg/ethers-flashbots/actions/workflows/ci.yml
[crates-badge]: https://img.shields.io/crates/v/ethers-flashbots.svg
[crates-url]: https://crates.io/crates/ethers-flashbots
[docs-badge]: https://docs.rs/ethers-flashbots/badge.svg
[docs-url]: https://docs.rs/ethers-flashbots

An [Ethers](https://github.com/gakonst/ethers-rs) middleware to send transactions as [Flashbots](https://docs.flashbots.net) bundles.

### Installation

Add `ethers-flashbots` to your `Cargo.toml`.

```toml
# This is the development version, for the stable release refer
# to crates.io
ethers-flashbots = { git = "https://github.com/onbjerg/ethers-flashbots" }
```

### Usage

```rs
use eyre::Result;
use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers_flashbots::*;
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
        .ok_or_else(|| eyre::format_err!("tx not included"))?;
    let tx = client.get_transaction(receipt.transaction_hash).await?;

    println!("Sent transaction: {}\n", serde_json::to_string(&tx)?);
    println!("Receipt: {}\n", serde_json::to_string(&receipt)?);

    Ok(())
}
```

See [the examples](./examples) for more in-depth examples.

### Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure that the tests and lints pass (`cargo test && cargo clippy -- -D clippy::all && cargo fmt -- --check`).

Make sure to add your changes to the "Unreleased" section of the changelog.

### Donate

If you would like to support me in my open source journey feel free to send me some Eth or tokens (anything accepted) at bjerg.eth. I appreciate it! 🙇

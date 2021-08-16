//! # Ethers Flashbots
//!
//! Provides an [ethers](https://docs.rs/ethers) compatible middleware for submitting
//! [Flashbots](https://docs.flashbots.net) bundles.
//!
//! In addition to leveraging the standard Ethers middleware API ([`send_transaction`][ethers_providers::Middleware::send_transaction]),
//! custom bundles can be crafted, simulated and submitted.
mod bundle;
pub use bundle::{
    BundleHash, BundleRequest, BundleTransaction, SimulatedBundle, SimulatedTransaction,
};

mod middleware;
pub use middleware::{FlashbotsMiddleware, FlashbotsMiddlewareError};

mod jsonrpc;
mod relay;
pub use relay::{Relay, RelayError};

mod utils;

use crate::{
    bundle::{BundleHash, BundleRequest, BundleStats, SimulatedBundle},
    pending_bundle::PendingBundle,
    relay::{GetBundleStatsParams, GetUserStatsParams, Relay, RelayError, SendBundleResponse},
    UserStats,
};
use async_trait::async_trait;
use ethers::core::{
    types::{BlockNumber, Bytes, U64},
    utils::keccak256,
};
use ethers::providers::{FromErr, Middleware, PendingTransaction};
use ethers::signers::Signer;
use thiserror::Error;
use url::Url;

/// Errors for the Flashbots middleware.
#[derive(Error, Debug)]
pub enum FlashbotsMiddlewareError<M: Middleware, S: Signer> {
    /// Some parameters were missing.
    ///
    /// For bundle simulation, check that the following are set:
    /// - `simulation_block`
    /// - `simulation_timestamp`
    /// - `block`
    ///
    /// For bundle submission, check that the following are set:
    /// - `block`
    ///
    /// Additionally, `min_timestamp` and `max_timestamp` must
    /// both be set or unset.
    #[error("Some parameters were missing")]
    MissingParameters,
    /// The relay responded with an error.
    #[error(transparent)]
    RelayError(#[from] RelayError<S>),
    /// An error occured in one of the middlewares.
    #[error("{0}")]
    MiddlewareError(M::Error),
}

impl<M: Middleware, S: Signer> FromErr<M::Error> for FlashbotsMiddlewareError<M, S> {
    fn from(err: M::Error) -> FlashbotsMiddlewareError<M, S> {
        FlashbotsMiddlewareError::MiddlewareError(err)
    }
}

/// A middleware used to send bundles to a Flashbots relay.
///
/// **NOTE**: This middleware does **NOT** sign your transactions. Use
/// another method to sign your transactions, and then forward the signed
/// transactions to the middleware.
///
/// You can either send custom bundles (see [`BundleRequest`]) or send
/// transactions as you normally would (see [`Middleware::send_transaction`]) from
/// another middleware.
///
/// If you use [`Middleware::send_transaction`] then a bundle will be constructed
/// for you with the following assumptions:
///
/// - You do not want to allow the transaction to revert
/// - You do not care to set a minimum or maximum timestamp for the bundle
/// - The block you are targetting with your bundle is the next block
/// - You do not want to simulate the bundle before sending to the relay
///
/// # Example
/// ```
/// use ethers::prelude::*;
/// use std::convert::TryFrom;
/// use ethers_flashbots::FlashbotsMiddleware;
/// use url::Url;
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = Provider::<Http>::try_from("http://localhost:8545")
///     .expect("Could not instantiate HTTP provider");
///
/// // Used to sign Flashbots relay requests - this is your searcher identity
/// let signer: LocalWallet = "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc"
///     .parse()?;
///
/// // Used to sign transactions
/// let wallet: LocalWallet = "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc"
///     .parse()?;
///
/// // Note: The order is important! You want the signer
/// // middleware to sign your transactions *before* they
/// // are sent to your Flashbots middleware.
/// let mut client = SignerMiddleware::new(
///     FlashbotsMiddleware::new(
///         provider,
///         Url::parse("https://relay.flashbots.net")?,
///         signer
///     ),
///     wallet
/// );
///
/// // This transaction will now be send as a Flashbots bundle!
/// let tx = TransactionRequest::pay("vitalik.eth", 100);
/// let pending_tx = client.send_transaction(tx, None).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct FlashbotsMiddleware<M, S> {
    inner: M,
    relay: Relay<S>,
}

impl<M: Middleware, S: Signer> FlashbotsMiddleware<M, S> {
    /// Initialize a new Flashbots middleware.
    ///
    /// The signer is used to sign requests to the relay.
    pub fn new(inner: M, relay_url: impl Into<Url>, relay_signer: S) -> Self {
        Self {
            inner,
            relay: Relay::new(relay_url, relay_signer),
        }
    }

    /// Get the relay client used by the middleware.
    pub fn relay(&self) -> &Relay<S> {
        &self.relay
    }

    /// Simulate a bundle.
    ///
    /// See [`eth_callBundle`][fb_callBundle] for more information.
    ///
    /// [fb_callBundle]: https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint#eth_callbundle
    pub async fn simulate_bundle(
        &self,
        bundle: &BundleRequest,
    ) -> Result<SimulatedBundle, FlashbotsMiddlewareError<M, S>> {
        bundle
            .block()
            .and(bundle.simulation_block())
            .and(bundle.simulation_timestamp())
            .ok_or(FlashbotsMiddlewareError::MissingParameters)?;

        self.relay
            .request("eth_callBundle", [bundle])
            .await
            .map_err(FlashbotsMiddlewareError::RelayError)
    }

    /// Send a bundle to the relayer.
    ///
    /// See [`eth_sendBundle`][fb_sendBundle] for more information.
    ///
    /// [fb_sendBundle]: https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint#eth_sendbundle
    pub async fn send_bundle(
        &self,
        bundle: &BundleRequest,
    ) -> Result<PendingBundle<'_, <Self as Middleware>::Provider>, FlashbotsMiddlewareError<M, S>>
    {
        // The target block must be set
        bundle
            .block()
            .ok_or(FlashbotsMiddlewareError::MissingParameters)?;

        // `min_timestamp` and `max_timestamp` must both either be unset or set.
        if bundle.min_timestamp().xor(bundle.max_timestamp()).is_some() {
            return Err(FlashbotsMiddlewareError::MissingParameters);
        }

        let response: SendBundleResponse = self
            .relay
            .request("eth_sendBundle", [bundle])
            .await
            .map_err(FlashbotsMiddlewareError::RelayError)?;

        Ok(PendingBundle::new(
            response.bundle_hash,
            bundle.block().unwrap(),
            bundle.transaction_hashes(),
            self.provider(),
        ))
    }

    /// Get stats for a particular bundle.
    pub async fn get_bundle_stats(
        &self,
        bundle_hash: BundleHash,
        block_number: U64,
    ) -> Result<BundleStats, FlashbotsMiddlewareError<M, S>> {
        self.relay
            .request(
                "flashbots_getBundleStats",
                [GetBundleStatsParams {
                    bundle_hash,
                    block_number,
                }],
            )
            .await
            .map_err(FlashbotsMiddlewareError::RelayError)
    }

    /// Get stats for your searcher identity.
    ///
    /// Your searcher identity is determined by the signer you
    /// constructed the middleware with.
    pub async fn get_user_stats(&self) -> Result<UserStats, FlashbotsMiddlewareError<M, S>> {
        let latest_block = self
            .inner
            .get_block_number()
            .await
            .map_err(FlashbotsMiddlewareError::MiddlewareError)?;

        self.relay
            .request(
                "flashbots_getUserStats",
                [GetUserStatsParams {
                    block_number: latest_block,
                }],
            )
            .await
            .map_err(FlashbotsMiddlewareError::RelayError)
    }
}

#[async_trait]
impl<M, S> Middleware for FlashbotsMiddleware<M, S>
where
    M: Middleware,
    S: Signer,
{
    type Error = FlashbotsMiddlewareError<M, S>;
    type Provider = M::Provider;
    type Inner = M;

    fn inner(&self) -> &M {
        &self.inner
    }

    async fn send_raw_transaction<'a>(
        &'a self,
        tx: Bytes,
    ) -> Result<PendingTransaction<'a, Self::Provider>, Self::Error> {
        let tx_hash = keccak256(&tx);

        // Get the latest block
        let latest_block = self
            .inner
            .get_block(BlockNumber::Latest)
            .await
            .map_err(FlashbotsMiddlewareError::MiddlewareError)?
            .expect("The latest block is pending (this should not happen)");

        // Construct the bundle, assuming that the target block is the
        // next block.
        let bundle = BundleRequest::new().push_transaction(tx.clone()).set_block(
            latest_block
                .number
                .expect("The latest block is pending (this should not happen)")
                + 1,
        );

        self.send_bundle(&bundle).await?;

        Ok(PendingTransaction::new(tx_hash.into(), self.provider())
            .interval(self.provider().get_interval()))
    }
}

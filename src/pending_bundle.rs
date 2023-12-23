use crate::bundle::BundleHash;
use ethers::core::types::{Block, TxHash, U64};
use ethers::providers::{
    interval, JsonRpcClient, Middleware, Provider, ProviderError, DEFAULT_POLL_INTERVAL,
};
use futures_core::stream::Stream;
use futures_util::stream::StreamExt;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use thiserror::Error;

/// A pending bundle is one that has been submitted to a relay,
/// but not yet included.
///
/// You can `await` the pending bundle. When the target block of the
/// bundle has been included in the chain the future will resolve,
/// either with the bundle hash indicating that the bundle was
/// included in the target block, or with an error indicating
/// that the bundle was not included in the target block.
///
/// To figure out why your bundle was not included, refer to the
/// [Flashbots documentation][fb_debug].
///
/// [fb_debug]: https://docs.flashbots.net/flashbots-auction/searchers/faq/#why-didnt-my-transaction-get-included
#[pin_project]
pub struct PendingBundle<'a, P> {
    pub bundle_hash: Option<BundleHash>,
    pub block: U64,
    pub transactions: Vec<TxHash>,
    provider: &'a Provider<P>,
    state: PendingBundleState<'a>,
    interval: Box<dyn Stream<Item = ()> + Send + Unpin>,
}

impl<'a, P: JsonRpcClient> PendingBundle<'a, P> {
    pub fn new(
        bundle_hash: Option<BundleHash>,
        block: U64,
        transactions: Vec<TxHash>,
        provider: &'a Provider<P>,
    ) -> Self {
        Self {
            bundle_hash,
            block,
            transactions,
            provider,
            state: PendingBundleState::PausedGettingBlock,
            interval: Box::new(interval(DEFAULT_POLL_INTERVAL)),
        }
    }

    /// Get the bundle hash for this pending bundle.
    #[deprecated(note = "use the bundle_hash field instead")]
    pub fn bundle_hash(&self) -> Option<BundleHash> {
        self.bundle_hash
    }
}

impl<'a, P: JsonRpcClient> Future for PendingBundle<'a, P> {
    type Output = Result<Option<BundleHash>, PendingBundleError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        match this.state {
            PendingBundleState::PausedGettingBlock => {
                futures_util::ready!(this.interval.poll_next_unpin(ctx));
                let fut = Box::pin(this.provider.get_block(*this.block));
                *this.state = PendingBundleState::GettingBlock(fut);
                ctx.waker().wake_by_ref();
            }
            PendingBundleState::GettingBlock(fut) => {
                let block_res = futures_util::ready!(fut.as_mut().poll(ctx));

                // If the provider errors, we try again after some interval.
                if block_res.is_err() {
                    *this.state = PendingBundleState::PausedGettingBlock;
                    ctx.waker().wake_by_ref();
                    return Poll::Pending;
                }

                let block_opt = block_res.unwrap();
                // If the block doesn't exist yet, we try again after some interval.
                if block_opt.is_none() {
                    *this.state = PendingBundleState::PausedGettingBlock;
                    ctx.waker().wake_by_ref();
                    return Poll::Pending;
                }

                let block = block_opt.unwrap();
                // If the block is pending, we try again after some interval.
                if block.number.is_none() {
                    *this.state = PendingBundleState::PausedGettingBlock;
                    ctx.waker().wake_by_ref();
                    return Poll::Pending;
                }

                // Check if all transactions of the bundle are present in the block
                let included: bool = this
                    .transactions
                    .iter()
                    .all(|tx_hash| block.transactions.contains(tx_hash));

                *this.state = PendingBundleState::Completed;
                if included {
                    return Poll::Ready(Ok(*this.bundle_hash));
                } else {
                    return Poll::Ready(Err(PendingBundleError::BundleNotIncluded));
                }
            }
            PendingBundleState::Completed => {
                panic!("polled pending bundle future after completion")
            }
        }

        Poll::Pending
    }
}

/// Errors for pending bundles.
#[derive(Error, Debug)]
pub enum PendingBundleError {
    /// The bundle was not included in the target block.
    #[error("Bundle was not included in target block")]
    BundleNotIncluded,
    /// An error occured while interacting with the RPC endpoint.
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}

type PinBoxFut<'a, T> = Pin<Box<dyn Future<Output = Result<T, ProviderError>> + Send + 'a>>;

enum PendingBundleState<'a> {
    /// Waiting for an interval before calling API again
    PausedGettingBlock,

    /// Polling the blockchain to get block information
    GettingBlock(PinBoxFut<'a, Option<Block<TxHash>>>),

    /// Future has completed
    Completed,
}

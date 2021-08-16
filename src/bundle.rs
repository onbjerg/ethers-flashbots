use ethers_core::types::{transaction::response::Transaction, Address, Bytes, H256, U256, U64};
use serde::{Deserialize, Serialize, Serializer};

/// A bundle hash.
pub type BundleHash = H256;

/// A transaction that can be added to a bundle.
#[derive(Debug, Clone)]
pub enum BundleTransaction {
    /// A pre-signed transaction.
    Signed(Transaction),
    /// An RLP encoded signed transaction.
    Raw(Bytes),
}

impl From<Transaction> for BundleTransaction {
    fn from(tx: Transaction) -> Self {
        Self::Signed(tx)
    }
}

impl From<Bytes> for BundleTransaction {
    fn from(tx: Bytes) -> Self {
        Self::Raw(tx)
    }
}

/// A bundle that can be submitted to a Flashbots relay.
///
/// The bundle can include your own transactions and transactions from
/// the mempool.
///
/// Additionally, this bundle can be simulated through a relay if simulation
/// parameters are provided using [`BundleRequest::set_simulation_block`] and
/// [`BundleRequest::set_simulation_timestamp`].
///
/// Please note that some parameters are required, and submitting a bundle
/// without them will get it rejected pre-flight. The required parameters
/// include:
///
/// - At least one transaction ([`BundleRequest::push_transaction`])
/// - A target block ([`BundleRequest::set_block`])
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleRequest {
    #[serde(rename = "txs")]
    #[serde(serialize_with = "serialize_txs")]
    transactions: Vec<BundleTransaction>,
    #[serde(skip)]
    revertible_transactions: Vec<usize>,
    #[serde(rename = "revertingTxHashes")]
    revertible_transaction_hashes: Vec<Bytes>,

    #[serde(rename = "blockNumber")]
    target_block: Option<U64>,
    min_timestamp: Option<u64>,
    max_timestamp: Option<u64>,

    #[serde(rename = "stateBlockNumber")]
    simulation_block: Option<U64>,
    #[serde(rename = "timestamp")]
    simulation_timestamp: Option<u64>,
}

pub fn serialize_txs<S>(txs: &[BundleTransaction], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw_txs: Vec<Bytes> = txs
        .iter()
        .map(|tx| match tx {
            BundleTransaction::Signed(inner) => inner.rlp(),
            BundleTransaction::Raw(inner) => inner.clone(),
        })
        .collect();

    raw_txs.serialize(s)
}

impl BundleRequest {
    /// Creates an empty bundle request.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a transaction to the bundle request.
    ///
    /// Transactions added to the bundle can either be novel transactions,
    /// i.e. transactions that you have crafted, or they can be from
    /// one of the mempool APIs.
    pub fn push_transaction<T: Into<BundleTransaction>>(mut self, tx: T) -> Self {
        self.transactions.push(tx.into());
        self
    }

    /// Adds a revertible transaction to the bundle request.
    ///
    /// This differs from [`BundleRequest::push_transaction`] in that the bund will still be
    /// considered valid if the transaction reverts.
    pub fn push_revertible_transaction<T: Into<BundleTransaction>>(mut self, tx: T) -> Self {
        self.transactions.push(tx.into());
        self.revertible_transactions
            .push(self.transactions.len() - 1);
        self
    }

    /// Get a reference to the transactions currently in the bundle request.
    pub fn transactions(&self) -> &Vec<BundleTransaction> {
        &self.transactions
    }

    /// Get the target block (if any).
    pub fn block(&self) -> Option<U64> {
        self.target_block
    }

    /// Set the target block of the bundle.
    pub fn set_block(mut self, block: U64) -> Self {
        self.target_block = Some(block);
        self
    }

    /// Get the block that determines the state for bundle simulation (if any).
    ///
    /// See [`eth_callBundle`][fb_call_bundle] in the Flashbots documentation
    /// for more information on bundle simulations.
    ///
    /// [fb_call_bundle]: https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint#eth_callbundle
    pub fn simulation_block(&self) -> Option<U64> {
        self.simulation_block
    }

    /// Set the block that determines the state for bundle simulation.
    pub fn set_simulation_block(mut self, block: U64) -> Self {
        self.simulation_block = Some(block);
        self
    }

    /// Get the UNIX timestamp used for bundle simulation (if any).
    ///
    /// See [`eth_callBundle`][fb_call_bundle] in the Flashbots documentation
    /// for more information on bundle simulations.
    ///
    /// [fb_call_bundle]: https://docs.flashbots.net/flashbots-auction/searchers/advanced/rpc-endpoint#eth_callbundle
    pub fn simulation_timestamp(&self) -> Option<u64> {
        self.simulation_timestamp
    }

    /// Set the UNIX timestamp used for bundle simulation.
    pub fn set_simulation_timestamp(mut self, timestamp: u64) -> Self {
        self.simulation_timestamp = Some(timestamp);
        self
    }

    /// Get the minimum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn min_timestamp(&self) -> Option<u64> {
        self.min_timestamp
    }

    /// Set the minimum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn set_min_timestamp(mut self, timestamp: u64) -> Self {
        self.min_timestamp = Some(timestamp);
        self
    }

    /// Get the maximum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn max_timestamp(&self) -> Option<u64> {
        self.max_timestamp
    }

    /// Set the maximum timestamp for which this bundle is valid (if any),
    /// in seconds since the UNIX epoch.
    pub fn set_max_timestamp(mut self, timestamp: u64) -> Self {
        self.max_timestamp = Some(timestamp);
        self
    }
}

/// Details of a simulated transaction.
///
/// Details for a transaction that has been simulated as part of
/// a bundle.
#[derive(Debug, Clone, Deserialize)]
pub struct SimulatedTransaction {
    /// The transaction hash
    #[serde(rename = "txHash")]
    pub hash: H256,
    /// The difference in coinbase's balance due to this transaction.
    ///
    /// This includes tips and gas fees for this transaction.
    #[serde(rename = "coinbaseDiff")]
    pub coinbase_diff: U256,
    /// The amount of Eth sent to coinbase in this transaction.
    #[serde(rename = "ethSentToCoinbase")]
    pub coinbase_tip: U256,
    /// The gas price.
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// The amount of gas used in this transaction.
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    /// The total gas fees for this transaction.
    #[serde(rename = "gasFees")]
    pub gas_fees: U256,
    /// The origin of this transaction.
    #[serde(rename = "fromAddress")]
    pub from: Address,
    /// The destination of this transaction.
    #[serde(rename = "toAddress")]
    pub to: Address,
    /// The value sent in this transaction.
    pub value: U256,
}

/// Details of a simulated bundle.
///
/// The details of a bundle that has been simulated.
#[derive(Debug, Clone, Deserialize)]
pub struct SimulatedBundle {
    /// The bundle's hash.
    #[serde(rename = "bundleHash")]
    pub hash: BundleHash,
    /// The difference in coinbase's balance due to this bundle.
    ///
    /// This includes total gas fees and coinbase tips.
    #[serde(rename = "coinbaseDiff")]
    pub coinbase_diff: U256,
    /// The amount of Eth sent to coinbase in this bundle.
    #[serde(rename = "ethSentToCoinbase")]
    pub coinbase_tip: U256,
    /// The gas price of the bundle.
    #[serde(rename = "bundleGasPrice")]
    pub gas_price: U256,
    /// The total amount of gas used in this bundle.
    #[serde(rename = "totalGasUsed")]
    pub gas_used: U256,
    /// The total amount of gas fees in this bundle.
    #[serde(rename = "gasFees")]
    pub gas_fees: U256,
    /// The block at which this bundle was simulated.
    #[serde(rename = "stateBlockNumber")]
    pub simulation_block: U64,
    /// The simulated transactions in this bundle.
    #[serde(rename = "results")]
    pub transactions: Vec<SimulatedTransaction>,
}

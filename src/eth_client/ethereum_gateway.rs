use std::fmt::Debug;

use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::Options;
use web3::types::{Address, BlockId, Filter, Log, U64};

use crate::config;
use crate::eth_client::clients::mock::MockEthereum;
use crate::types::{TransactionReceipt, H160, H256, U256};

#[derive(Debug, Clone, PartialEq)]
pub struct SignedCallResult {
    pub raw_tx: Vec<u8>,
    pub gas_price: U256,
    pub nonce: U256,
    pub hash: H256,
}

/// State of the executed Ethereum transaction.
#[derive(Debug, Clone)]
pub struct ExecutedTxStatus {
    /// Amount of confirmations for a block containing the transaction.
    pub confirmations: u64,
    /// Whether transaction was executed successfully or failed.
    pub success: bool,
    /// Receipt for a transaction. Will be set to `Some` only if the transaction
    /// failed during execution.
    pub receipt: Option<TransactionReceipt>,
}
/// Information about transaction failure.
#[derive(Debug, Clone)]
pub struct FailureInfo {
    pub revert_code: String,
    pub revert_reason: String,
    pub gas_used: Option<U256>,
    pub gas_limit: U256,
}

#[derive(Debug, Clone)]
pub enum EthereumGateway {
    // TODO:
    Mock(MockEthereum),
}

impl EthereumGateway {
    pub fn from_config(config: &config::Settings) -> Self {
        // TODO:
        Self::Mock(MockEthereum::default())
    }
}

macro_rules! delegate_call {
    ($self:ident.$method:ident($($args:ident),+)) => {
        match $self {
            // Self::Direct(d) => d.$method($($args),+).await,
            // Self::Multiplexed(d) => d.$method($($args),+).await,
            Self::Mock(d) => d.$method($($args),+).await,
        }
    };
    ($self:ident.$method:ident()) => {
        match $self {
            // Self::Direct(d) => d.$method().await,
            // Self::Multiplexed(m) => m.$method().await,
            Self::Mock(d) => d.$method().await,
        }
    }

}

impl EthereumGateway {
    /// Returns the next *expected* nonce with respect to the transactions
    /// in the mempool.
    ///
    /// Note that this method may be inconsistent if used with a cluster of nodes
    /// (e.g. `infura`), since the consecutive tx send and attempt to get a pending
    /// nonce may be routed to the different nodes in cluster, and the latter node
    /// may not know about the send tx yet. Thus it is not recommended to rely on this
    /// method as on the trusted source of the latest nonce.
    pub async fn pending_nonce(&self) -> Result<U256, anyhow::Error> {
        delegate_call!(self.pending_nonce())
    }

    /// Returns the account nonce based on the last *mined* block. Not mined transactions
    /// (which are in mempool yet) are not taken into account by this method.
    pub async fn current_nonce(&self) -> Result<U256, anyhow::Error> {
        delegate_call!(self.current_nonce())
    }

    pub async fn block_number(&self) -> Result<U64, anyhow::Error> {
        delegate_call!(self.block_number())
    }

    pub async fn get_gas_price(&self) -> Result<U256, anyhow::Error> {
        delegate_call!(self.get_gas_price())
    }
    /// Returns the account balance.
    pub async fn sender_eth_balance(&self) -> Result<U256, anyhow::Error> {
        delegate_call!(self.sender_eth_balance())
    }

    /// Signs the transaction given the previously encoded data.
    /// Fills in gas/nonce if not supplied inside options.
    pub async fn sign_prepared_tx(&self, data: Vec<u8>, options: Options) -> Result<SignedCallResult, anyhow::Error> {
        delegate_call!(self.sign_prepared_tx(data, options))
    }

    /// Signs the transaction given the previously encoded data.
    /// Fills in gas/nonce if not supplied inside options.
    pub async fn sign_prepared_tx_for_addr(
        &self,
        data: Vec<u8>,
        contract_addr: H160,
        options: Options,
    ) -> Result<SignedCallResult, anyhow::Error> {
        delegate_call!(self.sign_prepared_tx_for_addr(data, contract_addr, options))
    }

    /// Sends the transaction to the Ethereum blockchain.
    /// Transaction is expected to be encoded as the byte sequence.
    pub async fn send_raw_tx(&self, tx: Vec<u8>) -> Result<H256, anyhow::Error> {
        delegate_call!(self.send_raw_tx(tx))
    }

    /// Gets the Ethereum transaction receipt.
    pub async fn tx_receipt(&self, tx_hash: H256) -> Result<Option<TransactionReceipt>, anyhow::Error> {
        delegate_call!(self.tx_receipt(tx_hash))
    }

    pub async fn failure_reason(&self, tx_hash: H256) -> Result<Option<FailureInfo>, anyhow::Error> {
        delegate_call!(self.failure_reason(tx_hash))
    }

    /// Auxiliary function that returns the balance of the account on Ethereum.
    pub async fn eth_balance(&self, address: Address) -> Result<U256, anyhow::Error> {
        delegate_call!(self.eth_balance(address))
    }

    pub async fn allowance(&self, token_address: Address, erc20_abi: ethabi::Contract) -> Result<U256, anyhow::Error> {
        delegate_call!(self.allowance(token_address, erc20_abi))
    }
    pub async fn get_tx_status(&self, hash: H256) -> anyhow::Result<Option<ExecutedTxStatus>> {
        delegate_call!(self.get_tx_status(hash))
    }
    /// Encodes the transaction data (smart contract method and its input) to the bytes
    /// without creating an actual transaction.
    pub async fn call_main_contract_function<R, A, B, P>(
        &self,
        func: &str,
        params: P,
        from: A,
        options: Options,
        block: B,
    ) -> Result<R, anyhow::Error>
    where
        R: Detokenize + Unpin,
        A: Into<Option<Address>> + Clone,
        B: Into<Option<BlockId>> + Clone,
        P: Tokenize + Clone,
    {
        delegate_call!(self.call_main_contract_function(func, params, from, options, block))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn call_contract_function<R, A, B, P>(
        &self,
        func: &str,
        params: P,
        from: A,
        options: Options,
        block: B,
        token_address: Address,
        erc20_abi: ethabi::Contract,
    ) -> Result<R, anyhow::Error>
    where
        R: Detokenize + Unpin,
        A: Into<Option<Address>> + Clone,
        B: Into<Option<BlockId>> + Clone,
        P: Tokenize + Clone,
    {
        delegate_call!(self.call_contract_function(func, params, from, options, block, token_address, erc20_abi))
    }

    pub async fn logs(&self, filter: Filter) -> anyhow::Result<Vec<Log>> {
        delegate_call!(self.logs(filter))
    }

    pub fn encode_tx_data<P: Tokenize + Clone>(&self, func: &str, params: P) -> Vec<u8> {
        match self {
            // EthereumGateway::Multiplexed(c) => c.encode_tx_data(func, params),
            // EthereumGateway::Direct(c) => c.encode_tx_data(func, params),
            EthereumGateway::Mock(c) => c.encode_tx_data(func, params),
        }
    }

    pub fn get_mut_mock(&mut self) -> Option<&mut MockEthereum> {
        match self {
            EthereumGateway::Mock(m) => Some(m),
            _ => None,
        }
    }
    pub fn get_mock(&self) -> Option<&MockEthereum> {
        match self {
            EthereumGateway::Mock(m) => Some(m),
            _ => None,
        }
    }
}

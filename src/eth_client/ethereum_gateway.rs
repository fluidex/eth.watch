use std::fmt::Debug;

use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::Options;
use web3::types::{Address, BlockId, Filter, Log, U64};

#[derive(Debug, Clone)]
pub enum EthereumGateway {
    // Direct(ETHDirectClient<PrivateKeySigner>),
    // Multiplexed(MultiplexerEthereumClient),
    // Mock(MockEthereum),
    Mock,
}

impl EthereumGateway {
    pub fn from_config() -> Self {
        EthereumGateway::Mock
    }
    // pub fn from_config(config: &ZkSyncConfig) -> Self {}
}

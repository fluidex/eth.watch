mod client;
pub use client::EthHttpClient;

// pub struct EthWatch<W: EthClient> {
//     client: W,
//     eth_state: ETHState,
//     /// All ethereum events are accepted after sufficient confirmations to eliminate risk of block reorg.
//     number_of_confirmations_for_event: u64,
//     mode: WatcherMode,
// }

// impl<W: EthClient> EthWatch<W> {
//     pub fn new(client: W, number_of_confirmations_for_event: u64) -> Self {
//         Self {
//             client,
//             eth_state: ETHState::default(),
//             mode: WatcherMode::Working,
//             number_of_confirmations_for_event,
//         }
//     }
// }

pub struct EthWatch {
    client: EthHttpClient,
    /// All ethereum events are accepted after sufficient confirmations to eliminate risk of block reorg.
    number_of_confirmations_for_event: u64,
}

// impl<W: EthClient> EthWatch<W> {
impl EthWatch {
    pub fn new(client: EthHttpClient, number_of_confirmations_for_event: u64) -> Self {
        Self {
            client,
            number_of_confirmations_for_event,
        }
    }
}

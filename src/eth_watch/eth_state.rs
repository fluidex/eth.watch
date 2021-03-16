// Built-in deps
use std::collections::HashMap;
// // External uses
// // Workspace deps
// use zksync_types::{PriorityOp, SerialId};
// // Local deps
// use super::received_ops::ReceivedPriorityOp;

/// Gathered state of the Ethereum network.
/// Contains information about the known token types and incoming
/// priority operations (such as `Deposit` and `FullExit`).
///
/// All the data held is intentionally made private: as it represents the
/// observed state of the contract on Ethereum, it should never be
/// "partially updated". The state is either updated completely, or not
/// updated at all.
#[derive(Debug, Default, Clone)]
pub struct ETHState {
    /// The last block of the Ethereum network known to the Ethereum watcher.
    last_ethereum_block: u64,
    // TODO: other fields
}

impl ETHState {
    pub fn new(
        last_ethereum_block: u64,
        // unconfirmed_queue: Vec<PriorityOp>,
        // priority_queue: HashMap<SerialId, ReceivedPriorityOp>,
    ) -> Self {
        Self {
            last_ethereum_block,
            // unconfirmed_queue,
            // priority_queue,
        }
    }

    pub fn last_ethereum_block(&self) -> u64 {
        self.last_ethereum_block
    }

    //     pub fn priority_queue(&self) -> &HashMap<u64, ReceivedPriorityOp> {
    //         &self.priority_queue
    //     }

    //     pub fn unconfirmed_queue(&self) -> &[PriorityOp] {
    //         &self.unconfirmed_queue
    //     }
}

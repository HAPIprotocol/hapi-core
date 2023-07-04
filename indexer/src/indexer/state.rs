use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum IndexingCursor {
    None,
    Block(u64),
    Transaction(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum IndexerState {
    /// App is initializing: reading config and recovering the persisted state
    Init,
    /// App is checking for updates: checking for new blocks and transactions (start from the last persisted block)
    CheckForUpdates { cursor: IndexingCursor },
    /// App is running: indexing changes
    Processing { tx: String },
    /// App is waiting: waiting for new blocks and transactions until timestamp
    Waiting { cursor: IndexingCursor, until: u64 },
    /// App is stopped: no more indexing, with exit message
    Stopped { message: String },
}

impl IndexerState {
    pub fn transition(&mut self, new_state: Self) -> bool {
        match self {
            // Already stopped, do nothing
            IndexerState::Stopped { .. } => false,

            // If the new state is the same as the current one, do nothing
            _ if self == &new_state => false,

            // Otherwise, change the state
            _ => {
                tracing::info!(
                    from = ?self,
                    to = ?new_state,
                    "State change",
                );
                *self = new_state;
                true
            }
        }
    }
}

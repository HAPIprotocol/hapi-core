use std::fmt::Display;

use super::jobs::IndexerJob;
use {
    anyhow::{anyhow, Result},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndexingCursor {
    None,
    Block(u64),
    Transaction(String),
}

impl TryFrom<IndexerJob> for IndexingCursor {
    type Error = anyhow::Error;

    fn try_from(value: IndexerJob) -> Result<Self> {
        match value {
            IndexerJob::Transaction(tx) => Ok(IndexingCursor::Transaction(tx)),
            IndexerJob::Log(log) => Ok(IndexingCursor::Block(
                log.block_number
                    .ok_or(anyhow!("Unable to parse block number"))?
                    .as_u64(),
            )),
        }
    }
}

impl Display for IndexingCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingCursor::None => write!(f, "None"),
            IndexingCursor::Block(block) => write!(f, "Block({})", block),
            IndexingCursor::Transaction(tx) => write!(f, "Transaction({})", tx),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) enum IndexerState {
    /// App is initializing: reading config and recovering the persisted state
    Init,
    /// App is checking for updates: checking for new blocks and transactions (start from the last persisted block)
    CheckForUpdates { cursor: IndexingCursor },
    /// App is running: indexing changes
    Processing { cursor: IndexingCursor },
    /// App is waiting: waiting for new blocks and transactions until timestamp
    Waiting { cursor: IndexingCursor, until: u64 },
    /// App is stopped: no more indexing, with exit message
    Stopped { message: String },
}

impl IndexerState {
    pub fn transition(&mut self, new_state: Self) -> bool {
        match self {
            // Already stopped, don't proceed
            IndexerState::Stopped { message } => {
                tracing::info!(message, "Indexer stopped");
                false
            }

            // If the new state is waiting, and the current state is also waiting, just move on
            IndexerState::Waiting { .. } if matches!(new_state, IndexerState::Waiting { .. }) => {
                true
            }

            // If the new state is processing, and the current state is also processing, just move on
            IndexerState::Processing { .. }
                if matches!(new_state, IndexerState::Processing { .. }) =>
            {
                *self = new_state;
                true
            }

            // Otherwise, change the state
            _ => {
                tracing::debug!(
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

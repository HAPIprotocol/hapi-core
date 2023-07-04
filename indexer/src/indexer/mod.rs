use {
    anyhow::Result,
    std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::time::sleep,
};

pub(crate) mod network;
pub(crate) use network::Network;

pub(crate) mod state;
use state::{IndexerState, IndexingCursor};

pub(crate) mod server;

#[derive(Debug)]
pub(crate) struct Indexer {
    /// Network that indexer operates on
    network: Network,
    /// Current state of the indexer
    state: Arc<Mutex<IndexerState>>,
    /// Stack of transactions to index
    tx_stack: VecDeque<String>,
}

impl Indexer {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            state: Arc::new(Mutex::new(IndexerState::Init)),
            tx_stack: VecDeque::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(new_state) = self.next().await? {
            if !self.lock_transition(new_state)? {
                break;
            }
        }

        Ok(())
    }

    fn lock_transition(&mut self, new_state: IndexerState) -> Result<bool> {
        Ok(self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {:?}", e))?
            .transition(new_state))
    }

    fn lock_state(&self) -> Result<IndexerState> {
        Ok(self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {:?}", e))?
            .clone())
    }

    async fn next(&mut self) -> Result<Option<IndexerState>> {
        let new_state = match self.lock_state()? {
            IndexerState::Init => Some(self.handle_init().await),
            IndexerState::CheckForUpdates { cursor } => {
                Some(self.handle_check_for_updates(cursor).await)
            }
            IndexerState::Processing { tx } => Some(self.handle_process(tx).await),
            IndexerState::Waiting { until, cursor } => {
                Some(self.handle_waiting(until, cursor).await)
            }
            IndexerState::Stopped { .. } => None,
        };

        match new_state {
            Some(Ok(new_state)) => Ok(Some(new_state)),
            Some(Err(error)) => {
                tracing::error!(?error, "State handling error");
                Ok(Some(IndexerState::Stopped {
                    message: "Error occured".to_string(),
                }))
            }
            None => Ok(None),
        }
    }

    #[tracing::instrument(name = "init", skip(self))]
    async fn handle_init(&self) -> Result<IndexerState> {
        let cursor = match self.network {
            Network::Ethereum | Network::Bsc => {
                tracing::info!("Reading last persisted block");
                fake_work(1).await;
                IndexingCursor::Block(1_234_567)
            }
            Network::Solana | Network::Bitcoin => {
                tracing::info!("Reading last persisted transaction");
                fake_work(1).await;
                IndexingCursor::Transaction(
                    "0x45ca1f20b51331991de9128606ced314740e3455e7a6c0e3fd4b216bddcfe582"
                        .to_string(),
                )
            }
            Network::Near => {
                tracing::info!("Reading last persisted block or transaction");
                fake_work(1).await;
                IndexingCursor::Block(1_234_567)
            }
        };

        Ok(IndexerState::CheckForUpdates { cursor })
    }

    #[tracing::instrument(name = "check_for_updates", skip(self))]
    async fn handle_check_for_updates(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        match self.network {
            Network::Ethereum | Network::Bsc => {
                tracing::info!(
                    "Use `eth_getLogs` to list all new transactions since {:?}",
                    cursor
                );
                fake_work(1).await;
                // TODO: if transactions are acquired in a loop, it must be controlled for the stop signal and break if it's true
                // TODO: push transactions to the stack
                // NOTE: oldest transactions go first
                if cursor == IndexingCursor::None {
                    {
                        let tx =
                            "0x45ca1f20b51331991de9128606ced314740e3455e7a6c0e3fd4b216bddcfe582";
                        tracing::info!("Found new transaction: {}", tx);
                        self.tx_stack.push_back(tx.to_string());
                    }
                    {
                        let tx =
                            "0x3a73d90a0586d4d5eee84c1d66859473f982ef6c20e1f45a43a3dced3b5ae112";
                        tracing::info!("Found new transaction: {}", tx);
                        self.tx_stack.push_back(tx.to_string());
                    }
                } else {
                    tracing::info!("No new transactions found");
                }
            }
            Network::Solana | Network::Bitcoin => {
                tracing::info!(
                    "Use `getSignaturesForAddress` to list all new transactions since {:?}",
                    cursor
                );
                fake_work(1).await;
                // TODO: if transactions are acquired in a loop, it must be controlled for the stop signal and break if it's true
                // TODO: push transactions to the stack
                // NOTE: oldest transactions go first
                self.tx_stack.push_back("3nmQRhiWRHLcvzCb9xsttuV1Q6tJXgXr4raM5vypwGct3E5u5jdHp2ZxtAWQ7JpQiiZdVFzTSHdd3KFeM4KxiRyY".to_string());
            }
            Network::Near => {
                tracing::info!("Use `EXPERIMENTAL_changes` for every block since {:?} to list all new transactions", cursor);
                fake_work(1).await;
                // TODO: if transactions are acquired in a loop, it must be controlled for the stop signal and break if it's true
                // TODO: push transactions to the stack
                // NOTE: oldest transactions go first
                self.tx_stack
                    .push_back("9yPwtfGw9p8e8f9ogvBmeCXnxugquPUucMUf4g9aQxJj".to_string());
            }
        }
        if let Some(tx) = self.tx_stack.pop_front() {
            Ok(IndexerState::Processing { tx })
        } else {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
            Ok(IndexerState::Waiting {
                until: (now + Duration::from_secs(5)).as_secs(),
                cursor,
            })
        }
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, tx: String) -> Result<IndexerState> {
        tracing::info!("Processing transaction: {}", tx);
        fake_work(1).await;

        if let Some(tx) = self.tx_stack.pop_front() {
            Ok(IndexerState::Processing { tx })
        } else {
            tracing::info!("No more transactions in stack");
            Ok(IndexerState::CheckForUpdates {
                cursor: IndexingCursor::Transaction(tx), //or block!,
            })
        }
    }

    #[tracing::instrument(name = "waiting", skip(self))]
    async fn handle_waiting(&mut self, until: u64, cursor: IndexingCursor) -> Result<IndexerState> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if now > until {
            Ok(IndexerState::CheckForUpdates { cursor })
        } else {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            Ok(IndexerState::Waiting { until, cursor })
        }
    }
}

async fn fake_work(seconds: u64) {
    sleep(Duration::from_secs(seconds)).await;
}

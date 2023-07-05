use {
    anyhow::Result,
    std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::time::sleep,
};

use crate::config::IndexerConfiguration;

pub(crate) mod network;
use ethers::{
    providers::Middleware,
    types::{Address, BlockNumber, Filter, H256},
};
pub(crate) use network::Network;

pub(crate) mod state;
pub(crate) use state::{IndexerState, IndexingCursor};

pub(crate) mod client;
pub(crate) use client::IndexerClient;

pub(crate) mod server;

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

#[derive(Debug)]
pub(crate) struct Indexer {
    /// Network that indexer operates on
    network: Network,

    /// Address of the indexed contract
    contract_address: String,

    /// Current state of the indexer
    state: Arc<Mutex<IndexerState>>,

    /// Stack of transactions to index
    tx_stack: VecDeque<String>,

    /// The number of milliseconds between wait checks
    wait_interval_ms: Duration,

    /// Abstract client to access blockchain data
    client: IndexerClient,
}

impl Indexer {
    pub fn new(cfg: IndexerConfiguration) -> Result<Self> {
        tracing::info!(network = ?cfg.network, "Initializing indexer");
        Ok(Self {
            network: cfg.network.clone(),
            contract_address: cfg.contract_address,
            wait_interval_ms: cfg.wait_interval_ms,
            state: Arc::new(Mutex::new(IndexerState::Init)),
            tx_stack: VecDeque::new(),
            client: IndexerClient::new(cfg.network, &cfg.rpc_node_url)?,
        })
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
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {e:?}"))?
            .transition(new_state))
    }

    fn lock_state(&self) -> Result<IndexerState> {
        Ok(self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {e:?}"))?
            .clone())
    }

    async fn next(&mut self) -> Result<Option<IndexerState>> {
        let new_state = match self.lock_state()? {
            IndexerState::Init => Some(self.handle_init().await),
            IndexerState::CheckForUpdates { cursor } => {
                Some(self.handle_check_for_updates(cursor).await)
            }
            IndexerState::Processing { cursor } => Some(self.handle_process(cursor).await),
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
            Network::Sepolia | Network::Ethereum | Network::Bsc => {
                tracing::info!("Reading last persisted block");
                // fake_work(1).await;
                // IndexingCursor::Block(1)
                IndexingCursor::None
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
        match &self.client {
            IndexerClient::Ethers(client) => match cursor.clone() {
                IndexingCursor::None => {
                    tracing::info!("No cursor found searching for the earliest block height");

                    let filter = Filter::default()
                        .from_block(BlockNumber::Earliest)
                        .to_block(BlockNumber::Latest)
                        .address(self.contract_address.parse::<Address>()?);

                    // TODO: make sure it'll work with thousands of transactions; maybe apply paging?
                    match client
                        .get_logs(&filter)
                        .await?
                        .iter()
                        .filter_map(|x| x.block_number.as_ref().map(|bn| bn.as_u64()))
                        .next()
                    {
                        Some(block_number) => {
                            tracing::info!(block_number, "Earliest block height found");
                            Ok(IndexerState::CheckForUpdates {
                                cursor: IndexingCursor::Block(block_number),
                            })
                        }
                        None => Ok(IndexerState::Stopped {
                            message: "No valid transactions found on the contract address"
                                .to_string(),
                        }),
                    }
                }
                IndexingCursor::Block(last_block) => {
                    let current = client.get_block_number().await?.as_u64();
                    if last_block < current {
                        tracing::info!(from = last_block, to = current, "New blocks found");

                        let filter = Filter::default()
                            .from_block(BlockNumber::Number(last_block.into()))
                            .to_block(BlockNumber::Number(current.into()))
                            .address(self.contract_address.parse::<Address>()?);

                        self.tx_stack.extend(
                            client
                                .get_logs(&filter)
                                .await?
                                .iter()
                                .filter_map(|x| {
                                    x.transaction_hash
                                        .as_ref()
                                        .map(|hash| format!("{:?}", hash))
                                })
                                .map(|tx| {
                                    tracing::info!(tx = tx, "Found transaction");
                                    tx
                                }),
                        );

                        Ok(IndexerState::Processing {
                            cursor: IndexingCursor::Block(current),
                        })
                    } else {
                        tracing::info!("No new events found, waiting for new blocks");
                        Ok(IndexerState::Waiting {
                            until: now()? + 10, // TODO: blockchain-specific backoff interval
                            cursor,
                        })
                    }
                }
                IndexingCursor::Transaction(hash) => {
                    match client.get_transaction(hash.parse::<H256>()?).await? {
                        Some(_) => {
                            tracing::info!(tx = hash, "Found transaction");
                            self.tx_stack.push_back(hash.clone());
                            Ok(IndexerState::Processing {
                                cursor: IndexingCursor::Transaction(hash),
                            })
                        }
                        None => {
                            tracing::error!(tx = hash, "Transaction not found");
                            Ok(IndexerState::Stopped {
                                message: format!("Transaction '{hash}' not found"),
                            })
                        }
                    }
                }
            },
            _ => todo!(),
        }
        // match self.network {
        //     Network::Sepolia | Network::Ethereum | Network::Bsc => {
        //         if let IndexerClient::Ethers(client) = &self.client {
        //         } else {
        //             Ok(IndexerState::Stopped {
        //                 message: "Client is not initialized".to_string(),
        //             })
        //         }
        //     }
        //     Network::Solana | Network::Bitcoin => {
        //         tracing::info!(
        //             "Use `getSignaturesForAddress` to list all new transactions since {:?}",
        //             cursor
        //         );
        //         self.tx_stack.push_back("3nmQRhiWRHLcvzCb9xsttuV1Q6tJXgXr4raM5vypwGct3E5u5jdHp2ZxtAWQ7JpQiiZdVFzTSHdd3KFeM4KxiRyY".to_string());
        //         todo!();
        //         // TODO: if transactions are acquired in a loop, it must be controlled for the stop signal and break if it's true
        //         // TODO: push transactions to the stack
        //         // NOTE: oldest transactions go first
        //     }
        //     Network::Near => {
        //         tracing::info!("Use `EXPERIMENTAL_changes` for every block since {:?} to list all new transactions", cursor);
        //         self.tx_stack
        //             .push_back("9yPwtfGw9p8e8f9ogvBmeCXnxugquPUucMUf4g9aQxJj".to_string());
        //         todo!();
        //         // TODO: if transactions are acquired in a loop, it must be controlled for the stop signal and break if it's true
        //         // TODO: push transactions to the stack
        //         // NOTE: oldest transactions go first
        //     }
        // }
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        match self.tx_stack.pop_front() {
            Some(tx) => {
                tracing::info!(tx, "Processing transaction");

                match &self.client {
                    IndexerClient::Ethers(client) => {
                        // TODO: handle duplicate transaction entries
                        let tx = client.get_transaction(tx.parse::<H256>()?).await?;
                        if let Some(tx) = tx {
                            tracing::debug!(
                                bytes = tx.input.len(),
                                ?tx.block_number,
                                "TODO: process transaction"
                            );
                        }
                    }
                    _ => todo!(),
                }

                Ok(IndexerState::Processing { cursor })
            }
            None => {
                tracing::trace!("No more transactions in stack");
                Ok(IndexerState::CheckForUpdates { cursor })
            }
        }
    }

    #[tracing::instrument(name = "waiting", skip(self))]
    async fn handle_waiting(&mut self, until: u64, cursor: IndexingCursor) -> Result<IndexerState> {
        if now()? > until {
            Ok(IndexerState::CheckForUpdates { cursor })
        } else {
            sleep(self.wait_interval_ms).await;
            Ok(IndexerState::Waiting { until, cursor })
        }
    }
}

async fn fake_work(seconds: u64) {
    sleep(Duration::from_secs(seconds)).await;
}

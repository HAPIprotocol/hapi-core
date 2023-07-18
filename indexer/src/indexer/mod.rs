use {
    anyhow::Result,
    ethers::{
        providers::Middleware,
        types::{Address, BlockNumber, Filter, Log, H256},
    },
    serde::{Deserialize, Serialize},
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::{Arc, Mutex},
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::time::sleep,
};

use crate::config::IndexerConfiguration;

pub(crate) mod client;
pub(crate) mod persistence;
pub(crate) mod server;
pub(crate) mod state;

pub(crate) use {
    client::IndexerClient,
    persistence::PersistedState,
    state::{IndexerState, IndexingCursor},
};

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

pub(crate) struct Indexer {
    /// Address of the indexed contract
    contract_address: String,

    /// Current state of the indexer
    state: Arc<Mutex<IndexerState>>,

    /// Stack of transactions to index
    jobs: VecDeque<IndexerJob>,

    /// The number of milliseconds between wait checks
    wait_interval_ms: Duration,

    /// Abstract client to access blockchain data
    client: IndexerClient,

    /// The file to persist the indexer state in
    state_file: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum IndexerJob {
    Transaction(String),
    Log(Log),
}

impl Indexer {
    pub fn new(cfg: IndexerConfiguration) -> Result<Self> {
        tracing::info!(network = ?cfg.network, "Initializing indexer");
        Ok(Self {
            contract_address: cfg.contract_address.clone(),
            wait_interval_ms: cfg.wait_interval_ms,
            state: Arc::new(Mutex::new(IndexerState::Init)),
            jobs: VecDeque::new(),
            client: IndexerClient::new(cfg.network, &cfg.rpc_node_url, &cfg.contract_address)?,
            state_file: PathBuf::from(cfg.state_file),
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
    async fn handle_init(&mut self) -> Result<IndexerState> {
        if let Ok(state) = PersistedState::from_file(&self.state_file) {
            tracing::info!("Found persisted state");

            if !state.jobs.is_empty() {
                tracing::info!(size = state.jobs.len(), "Found persisted transaction stack");
                self.jobs = state.jobs;
            }

            if state.cursor != IndexingCursor::None {
                tracing::info!(cursor = ?state.cursor, "Found persisted cursor");
                return Ok(IndexerState::CheckForUpdates {
                    cursor: state.cursor,
                });
            }
        }

        Ok(IndexerState::CheckForUpdates {
            cursor: IndexingCursor::None,
        })
    }

    #[tracing::instrument(name = "check_for_updates", skip(self))]
    async fn handle_check_for_updates(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        match &self.client {
            IndexerClient::Evm(client) => match cursor.clone() {
                IndexingCursor::None => {
                    tracing::info!("No cursor found searching for the earliest block height");

                    let filter = Filter::default()
                        .from_block(BlockNumber::Earliest)
                        .to_block(BlockNumber::Latest)
                        .address(self.contract_address.parse::<Address>()?);

                    // TODO: make sure it'll work with thousands of transactions; maybe apply paging?
                    match client
                        .contract
                        .client()
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
                    let current = client.contract.client().get_block_number().await?.as_u64();
                    if last_block < current {
                        tracing::info!(from = last_block, to = current, "New blocks found");

                        let filter = Filter::default()
                            .from_block(BlockNumber::Number(last_block.into()))
                            .to_block(BlockNumber::Number(current.into()))
                            .address(self.contract_address.parse::<Address>()?);

                        self.jobs.extend(
                            client
                                .contract
                                .client()
                                .get_logs(&filter)
                                .await?
                                .into_iter()
                                .filter_map(|log| {
                                    if let Some(event_signature) = log.topics.first() {
                                        if let Some(name) = client
                                            .get_event_name_from_signature(H256(event_signature.0))
                                        {
                                            let tokens = client.contract.decode_event_raw(
                                                &name,
                                                log.topics.clone(),
                                                log.data.clone(),
                                            );
                                            tracing::info!(name, tx = ?log.transaction_hash, block = ?log.block_number, ?tokens, "Found event");

                                            Some(IndexerJob::Log(log))
                                        } else {
                                            tracing::warn!(tx = ?log.transaction_hash, block = ?log.block_number, "Unknown event signature");
                                            None
                                        }
                                    }
                                    else {
                                        tracing::warn!(tx = ?log.transaction_hash, block = ?log.block_number, "Event without signature");
                                        None
                                    }
                                })
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
                    match client
                        .contract
                        .client()
                        .get_transaction(hash.parse::<H256>()?)
                        .await?
                    {
                        Some(_) => {
                            tracing::info!(tx = hash, "Found transaction");
                            self.jobs.push_back(IndexerJob::Transaction(hash.clone()));
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
        match self.jobs.pop_front() {
            Some(IndexerJob::Transaction(tx)) => {
                tracing::info!(tx, "Processing transaction");

                match &self.client {
                    IndexerClient::Evm(client) => {
                        // TODO: handle duplicate transaction entries
                        let tx = client
                            .contract
                            .client()
                            .get_transaction(tx.parse::<H256>()?)
                            .await?;

                        if let Some(tx) = tx {
                            tracing::debug!(
                                bytes = tx.input.len(),
                                ?tx.block_number,
                                "TODO: process transaction"
                            );

                            match client.contract.decode_input_raw(tx.input) {
                                Ok(d) => {
                                    // TODO: act on correctly decoded input
                                    dbg!(&d);
                                }
                                Err(e) => {
                                    tracing::warn!(?e, "Failed to decode input");
                                }
                            }
                        }
                    }
                    _ => todo!(),
                }

                PersistedState {
                    cursor: cursor.clone(),
                    jobs: self.jobs.clone(),
                }
                .to_file(&self.state_file)?;

                Ok(IndexerState::Processing { cursor })
            }
            Some(IndexerJob::Log(log)) => {
                match &self.client {
                    IndexerClient::Evm(client) => {
                        if let Some(event_signature) = log.topics.first() {
                            if let Some(name) =
                                client.get_event_name_from_signature(H256(event_signature.0))
                            {
                                if let Ok(tokens) = client.contract.decode_event_raw(
                                    &name,
                                    log.topics.clone(),
                                    log.data.clone(),
                                ) {
                                    tracing::info!(name, tx = ?log.transaction_hash, block = ?log.block_number, ?tokens, "Found event");

                                    match name.as_str() {
                                        "ReporterCreated"
                                        | "ReporterUpdated"
                                        | "ReporterActivated"
                                        | "ReporterDeactivated"
                                        | "ReporterStakeWithdrawn" => {
                                            if let Some(reporter_id) = tokens.first() {
                                                tracing::info!(
                                                    ?reporter_id,
                                                    "Reporter is created or modified"
                                                );
                                            }
                                        }
                                        "CaseCreated" | "CaseUpdated" => {
                                            if let Some(case_id) = tokens.first() {
                                                tracing::info!(
                                                    ?case_id,
                                                    "Case is created or modified"
                                                );
                                            }
                                        }
                                        "AddressCreated" | "AddressUpdated" => {
                                            if let Some(addr) = tokens.first() {
                                                tracing::info!(
                                                    ?addr,
                                                    "Address is created or modified"
                                                );
                                            }
                                        }
                                        "AssetCreated" | "AssetUpdated" => {
                                            if let (Some(addr), Some(id)) =
                                                (tokens.get(0), tokens.get(1))
                                            {
                                                tracing::info!(
                                                    ?addr,
                                                    ?id,
                                                    "Asset is created or modified"
                                                );
                                            }
                                        }
                                        "AuthorityChanged"
                                        | "StakeConfigurationChanged"
                                        | "RewardConfigurationChanged" => {
                                            tracing::info!("Configuration is changed");
                                        }
                                        _ => {
                                            tracing::warn!("Uknown event");
                                        }
                                    }
                                } else {
                                    tracing::warn!(tx = ?log.transaction_hash, block = ?log.block_number, "Failed to decode event");
                                }
                            } else {
                                tracing::warn!(tx = ?log.transaction_hash, block = ?log.block_number, "Unknown event signature");
                            }
                        } else {
                            tracing::warn!(tx = ?log.transaction_hash, block = ?log.block_number, "No event signature");
                        }
                    }
                    _ => unimplemented!(),
                }

                PersistedState {
                    cursor: cursor.clone(),
                    jobs: self.jobs.clone(),
                }
                .to_file(&self.state_file)?;

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
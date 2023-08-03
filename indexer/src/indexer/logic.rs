use {
    anyhow::Result,
    ethers::{
        providers::Middleware,
        types::{Address, BlockNumber, Filter, H256},
    },
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::{Arc, Mutex},
    },
    tokio::time::sleep,
};

use hapi_core::HapiCoreEvm;

use crate::config::IndexerConfiguration;

use super::{
    now, Indexer, IndexerClient, IndexerJob, IndexerState, IndexingCursor, PersistedState,
};

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
            web_client: reqwest::Client::new(),
            webhook_url: cfg.webhook_url,
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
                tracing::info!(size = state.jobs.len(), "Found jobs in the queue");
                self.jobs = state.jobs;
            }

            if state.cursor != IndexingCursor::None {
                tracing::info!(cursor = ?state.cursor, "Found cursor");
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
        match (&self.client, cursor.clone()) {
            (IndexerClient::Evm(client), IndexingCursor::None) => {
                self.handle_update_evm_empty_cursor(client).await
            }
            (IndexerClient::Evm(client), IndexingCursor::Block(last_block)) => {
                let (state, new_jobs) = self.handle_update_evm_block(client, last_block).await?;

                self.jobs.extend(new_jobs);

                Ok(state)
            }
            (IndexerClient::Evm(client), IndexingCursor::Transaction(hash)) => {
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
            _ => unimplemented!(),
        }
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        match (self.jobs.pop_front(), &self.client) {
            (Some(IndexerJob::Log(log)), IndexerClient::Evm(client)) => {
                self.process_evm_job_log(client, &log).await?;
            }
            (Some(job), ..) => {
                tracing::warn!(?job, "Unsupported job type");
            }
            (None, ..) => {
                tracing::trace!("No more jobs in the queue");
            }
        };

        PersistedState {
            cursor: cursor.clone(),
            jobs: self.jobs.clone(),
        }
        .to_file(&self.state_file)?;

        Ok(IndexerState::Processing { cursor })
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

    async fn handle_update_evm_empty_cursor(&self, client: &HapiCoreEvm) -> Result<IndexerState> {
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
                message: "No valid transactions found on the contract address".to_string(),
            }),
        }
    }

    async fn handle_update_evm_block(
        &self,
        client: &HapiCoreEvm,
        last_block: u64,
    ) -> Result<(IndexerState, Vec<IndexerJob>)> {
        let current = client.contract.client().get_block_number().await?.as_u64();
        if last_block < current {
            tracing::info!(from = last_block, to = current, "New blocks found");

            let filter = Filter::default()
                .from_block(BlockNumber::Number(last_block.into()))
                .to_block(BlockNumber::Number(current.into()))
                .address(self.contract_address.parse::<Address>()?);

            let jobs = client
                .contract
                .client()
                .get_logs(&filter)
                .await?
                .into_iter()
                .filter_map(|log| match client.decode_event(&log) {
                    Ok(log_header) => {
                        tracing::info!(
                            name = log_header.name,
                            tx = ?log.transaction_hash,
                            block = ?log.block_number,
                            tokens = ?log_header.tokens,
                            "Found event",
                        );
                        Some(IndexerJob::Log(log))
                    }
                    Err(error) => {
                        tracing::warn!(
                            tx = ?log.transaction_hash,
                            block = ?log.block_number,
                            error = ?error,
                            "Event signature decoding error",
                        );
                        None
                    }
                });

            Ok((
                IndexerState::Processing {
                    cursor: IndexingCursor::Block(current),
                },
                jobs.collect(),
            ))
        } else {
            tracing::debug!("No new events found, waiting for new blocks");
            Ok((
                IndexerState::Waiting {
                    until: now()? + client.provider.get_interval().as_secs(),
                    cursor: IndexingCursor::Block(last_block),
                },
                Vec::new(),
            ))
        }
    }
}

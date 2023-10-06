use {
    anyhow::Result,
    ethers::types::Address,
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::{Arc, Mutex},
    },
    tokio::time::sleep,
};

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
        // TODO: make it more universal for all networks
        let contract_address = self.contract_address.parse::<Address>()?;

        match cursor {
            IndexingCursor::None => self.client.handle_update_cursor(contract_address).await,
            IndexingCursor::Block(last_block) => {
                let (state, new_jobs) = self
                    .client
                    .handle_update_block(last_block, contract_address)
                    .await?;

                self.jobs.extend(new_jobs);

                Ok(state)
            }
            IndexingCursor::Transaction(hash) => {
                let state = self.client.handle_update_transaction(hash.clone()).await?;
                if let IndexerState::Processing { .. } = state {
                    self.jobs.push_back(IndexerJob::Transaction(hash));
                }

                Ok(state)
            }
        }
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        // match (self.jobs.pop_front(), &self.client) {
        //     (Some(IndexerJob::Log(log)), IndexerClient::Evm(client)) => {
        //         self.process_evm_job_log(client, &log).await?;
        //     }
        //     (Some(job), ..) => {
        //         tracing::warn!(?job, "Unsupported job type");
        //     }
        //     (None, ..) => {
        //         tracing::trace!("No more jobs in the queue");
        //         // TODO: check if there are new blocks
        //         // Ok(IndexerState::CheckForUpdates { cursor })
        //     }
        // };

        match self.jobs.pop_front() {
            Some(job) => {
                if let Some(payload) = self.client.handle_process(job).await? {
                    self.send_webhook(&payload).await?;
                }
            }
            None => {
                tracing::trace!("No more jobs in the queue");
                // TODO: check if there are new blocks
                // Ok(IndexerState::CheckForUpdates { cursor })
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
}

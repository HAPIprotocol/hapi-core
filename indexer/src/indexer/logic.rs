use {
    anyhow::{bail, Result},
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::{Arc, Mutex},
    },
    tokio::time::sleep,
};

use crate::configuration::IndexerConfiguration;

use super::{
    now, Indexer, IndexerClient, IndexerJob, IndexerState, IndexingCursor, PersistedState,
};

impl Indexer {
    pub fn new(cfg: IndexerConfiguration) -> Result<Self> {
        tracing::info!(network = ?cfg.network, "Initializing indexer");
        Ok(Self {
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
            if !self.check_transition(new_state)? {
                break;
            }
        }

        Ok(())
    }

    fn check_transition(&mut self, new_state: IndexerState) -> Result<bool> {
        Ok(self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {e:?}"))?
            .transition(new_state))
    }

    fn get_state(&self) -> Result<IndexerState> {
        Ok(self
            .state
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {e:?}"))?
            .clone())
    }

    async fn next(&mut self) -> Result<Option<IndexerState>> {
        let new_state = match self.get_state()? {
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

    fn get_updated_state(
        &self,
        jobs: &Vec<IndexerJob>,
        old_cursor: IndexingCursor,
    ) -> Result<IndexerState> {
        if let Some(job) = jobs.first() {
            let cursor = IndexingCursor::try_from(job.clone())?;

            tracing::info!(%cursor, "Earliest cursor found");

            return Ok(IndexerState::Processing { cursor });
        } else if old_cursor == IndexingCursor::None {
            return Ok(IndexerState::Stopped {
                message: "No valid transactions found on the contract address".to_string(),
            });
        }

        Ok(IndexerState::Waiting {
            until: now()? + self.wait_interval_ms.as_secs(),
            cursor: old_cursor,
        })
    }

    #[tracing::instrument(name = "check_for_updates", skip(self))]
    async fn handle_check_for_updates(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        let new_jobs = self.client.fetch_jobs(&cursor).await?;
        let state = self.get_updated_state(&new_jobs, cursor)?;

        self.jobs.extend(new_jobs);

        Ok(state)
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        if let Some(job) = self.jobs.pop_front() {
            if let Some(payload) = self.client.handle_process(job).await? {
                for event in payload {
                    self.send_webhook(&event).await?;
                }
            }

            if let Some(next_job) = self.jobs.front() {
                let new_cursor = IndexingCursor::try_from(next_job.clone())?;

                PersistedState {
                    cursor: new_cursor.clone(),
                    jobs: self.jobs.clone(),
                }
                .to_file(&self.state_file)?;

                return Ok(IndexerState::Processing { cursor: new_cursor });
            } else {
                tracing::trace!("No more jobs in the queue");
                return Ok(IndexerState::CheckForUpdates { cursor });
            }
        };

        bail!("Processing an empty queue")
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

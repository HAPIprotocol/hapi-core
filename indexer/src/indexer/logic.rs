use {
    anyhow::{bail, Result},
    std::{collections::VecDeque, path::PathBuf, sync::Arc},
    tokio::{sync::Mutex, time::sleep},
};

use crate::{configuration::IndexerConfiguration, indexer::jwt::get_id_from_jwt};

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
            client: IndexerClient::new(
                get_id_from_jwt(&cfg.jwt_token)?,
                cfg.network,
                &cfg.rpc_node_url,
                &cfg.contract_address,
                cfg.fetching_delay,
            )?,
            state_file: PathBuf::from(cfg.state_file),
            web_client: reqwest::Client::new(),
            webhook_url: cfg.webhook_url,
            jwt_token: cfg.jwt_token,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let new_state = self.next().await?;

            if !self.check_transition(new_state).await {
                break;
            }
        }

        Ok(())
    }

    async fn check_transition(&mut self, new_state: IndexerState) -> bool {
        self.state.lock().await.transition(new_state)
    }

    async fn get_state(&self) -> IndexerState {
        self.state.lock().await.clone()
    }

    async fn next(&mut self) -> Result<IndexerState> {
        match self.get_state().await {
            IndexerState::Init => self.handle_init().await,
            IndexerState::CheckForUpdates { cursor } => self.handle_check_for_updates(cursor).await,
            IndexerState::Processing { cursor } => self.handle_process(cursor).await,
            IndexerState::Waiting { until, cursor } => self.handle_waiting(until, cursor).await,
            IndexerState::Stopped { .. } => bail!("Stopped indexer should not be running"),
        }
    }

    #[tracing::instrument(name = "init", skip(self))]
    async fn handle_init(&mut self) -> Result<IndexerState> {
        if let Ok(state) = PersistedState::from_file(&self.state_file) {
            tracing::info!("Found persisted state");

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
        jobs: &[IndexerJob],
        old_cursor: IndexingCursor,
        new_cursor: IndexingCursor,
    ) -> Result<IndexerState> {
        if !jobs.is_empty() {
            tracing::info!(%new_cursor, "Earliest cursor found");

            Ok(IndexerState::Processing { cursor: new_cursor })
        } else if old_cursor == IndexingCursor::None {
            Ok(IndexerState::Stopped {
                message: "No valid transactions found on the contract address".to_string(),
            })
        } else {
            let timestamp = now()? + self.wait_interval_ms.as_secs();
            tracing::info!(timestamp, %new_cursor, "New jobs not found, waiting until next check");

            PersistedState {
                cursor: new_cursor.clone(),
            }
            .to_file(&self.state_file)?;

            Ok(IndexerState::Waiting {
                until: timestamp,
                cursor: new_cursor,
            })
        }
    }

    #[tracing::instrument(name = "check_for_updates", skip(self))]
    async fn handle_check_for_updates(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        let artifacts = self.client.fetch_jobs(&cursor).await?;
        let state = self.get_updated_state(&artifacts.jobs, cursor, artifacts.cursor.clone())?;

        self.jobs.extend(artifacts.jobs);

        Ok(state)
    }

    #[tracing::instrument(name = "process", skip(self))]
    async fn handle_process(&mut self, cursor: IndexingCursor) -> Result<IndexerState> {
        if let Some(job) = self.jobs.pop_front() {
            if let Some(payload) = self.client.handle_process(&job).await? {
                for event in payload {
                    self.send_webhook(&event).await?;
                }
            }

            let new_cursor = IndexingCursor::try_from(job.clone())?;

            PersistedState {
                cursor: new_cursor.clone(),
            }
            .to_file(&self.state_file)?;

            return Ok(IndexerState::Processing { cursor });
        };

        PersistedState {
            cursor: cursor.clone(),
        }
        .to_file(&self.state_file)?;

        tracing::trace!("No more jobs in the queue");

        Ok(IndexerState::CheckForUpdates { cursor })
    }

    #[tracing::instrument(name = "waiting", skip(self))]
    async fn handle_waiting(&mut self, until: u64, cursor: IndexingCursor) -> Result<IndexerState> {
        self.send_heartbeat(&cursor).await?;

        if now()? > until {
            Ok(IndexerState::CheckForUpdates { cursor })
        } else {
            sleep(self.wait_interval_ms).await;
            Ok(IndexerState::Waiting { until, cursor })
        }
    }
}

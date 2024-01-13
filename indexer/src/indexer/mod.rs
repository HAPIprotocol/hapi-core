use {
    anyhow::Result,
    std::{
        collections::VecDeque,
        path::PathBuf,
        sync::Arc,
        time::{Duration, SystemTime, UNIX_EPOCH},
    },
    tokio::sync::Mutex,
};

pub(crate) mod client;
pub(crate) mod heartbeat;
pub(crate) mod jobs;
pub(crate) mod jwt;
pub(crate) mod logic;
pub(crate) mod persistence;
pub(crate) mod push;
pub(crate) mod server;
pub(crate) mod state;

pub(crate) use {
    client::IndexerClient,
    jobs::IndexerJob,
    persistence::PersistedState,
    state::{IndexerState, IndexingCursor},
};

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

pub struct Indexer {
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

    /// The HTTP client to use for webhooks
    web_client: reqwest::Client,

    /// The URL to send webhooks to
    webhook_url: String,

    /// JWT token to use for webhooks
    jwt_token: String,
}

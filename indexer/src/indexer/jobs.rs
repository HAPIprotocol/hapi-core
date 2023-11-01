use {
    ethers::types::Log,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum IndexerJob {
    Transaction(String),
    Log(Log),
}

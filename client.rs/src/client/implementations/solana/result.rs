use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaClientError {
    #[error("URL parse error: {0}")]
    UrlParseError(String),
    #[error("Solana address parse error: {0}")]
    AddressParseError(String),
    #[error("Unable to identify default solana config")]
    AbsentDefaultConfig,
    #[error("Unable to load solana config: {0}")]
    UnableToLoadConfig(String),
    #[error("Unable to read keypair file: {0}")]
    SolanaKeypairFile(String),
    #[error("Unable to initialize client: {0}")]
    RpcError(#[from] anchor_client::ClientError),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type SolanaClientResult<T> = std::result::Result<T, SolanaClientError>;

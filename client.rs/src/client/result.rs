// use crate::client::solana::result::SolanaClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("URL parse error: {0}")]
    UrlParseError(String),
    #[error("Asset Id parse error: {0}")]
    AssetIdParseError(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Failed to parse balance: {0}")]
    FailedToParseBalance(String),
    #[error("The reporter does not exist")]
    InvalidReporter,

    // Ethereum client errors
    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("ETH address parse error: {0}")]
    EthAddressParse(String),
    #[error("Ethers error: {0}")]
    Ethers(String),
    #[error("Provider error: {0}")]
    Provider(#[from] ethers_providers::ProviderError),
    #[error("Contract data parsing error: {0}")]
    ContractData(String),

    // Solana client errors
    #[error("Solana address parse error: {0}")]
    SolanaAddressParseError(String),
    #[error("Unable to identify default solana config")]
    AbsentDefaultConfig,
    #[error("Unable to load solana config: {0}")]
    UnableToLoadConfig(String),
    #[error("Unable to read keypair file: {0}")]
    SolanaKeypairFile(String),
    #[error("Anchor Rpc error: {0}")]
    // TODO: try to remove
    AnchorRpcError(#[from] anchor_client::ClientError),
    #[error("Solana Rpc error: {0}")]
    SolanaRpcError(#[from] anchor_client::solana_client::client_error::ClientError),
    #[error("This owner has no token account")]
    AbsentTokenAccount,
    #[error("Account not found")]
    AccountNotFound,
    #[error("Account deserialization error: {0}")]
    AccountDeserializationError(String),
    #[error("Solana token error: {0}")]
    SolanaTokenError(#[from] anchor_client::solana_sdk::program_error::ProgramError),
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone, Debug)]
pub struct Tx {
    pub hash: String,
}

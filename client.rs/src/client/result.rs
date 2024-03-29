use anchor_client::solana_sdk::signature::ParseSignatureError;
use near_jsonrpc_client::methods::broadcast_tx_async::RpcBroadcastTxAsyncError;
use near_jsonrpc_primitives::types::{query::RpcQueryError, transactions::RpcTransactionError};
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

    // Near client errors
    #[error("ParseAccountError error: {0}")]
    ParseAccountError(#[from] near_primitives::account::id::ParseAccountError),
    #[error("TimeoutError error: {0}")]
    TimeoutError(String),
    #[error("Error parse signer PK")]
    SignerError,
    #[error("Near RPC error: {0}")]
    RpcQueryError(#[from] near_jsonrpc_client::errors::JsonRpcError<RpcQueryError>),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Near request error: {0}")]
    NearRequestError(#[from] near_jsonrpc_client::errors::JsonRpcError<RpcBroadcastTxAsyncError>),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("RpcTransactionError error: {0}")]
    RpcTransactionError(#[from] near_jsonrpc_client::errors::JsonRpcError<RpcTransactionError>),

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
    #[error("Solana parse signature error: {0}")]
    ParseSignatureError(#[from] ParseSignatureError),
    #[error("Account instruction decoding: {0}")]
    InstructionDecodingError(String),
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone, Debug)]
pub struct Tx {
    pub hash: String,
}

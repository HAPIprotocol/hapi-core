use near_jsonrpc_client::methods::broadcast_tx_async::RpcBroadcastTxAsyncError;
use near_jsonrpc_primitives::types::{query::RpcQueryError, transactions::RpcTransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("URL parse error: {0}")]
    UrlParseError(String),
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
    #[error("ParseAccountError error: {0}")]
    ParseAccountError(#[from] near_primitives::account::id::ParseAccountError),
    #[error("TimeoutError error: {0}")]
    TimeoutError(String),
    #[error("Signer error: ")]
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
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone, Debug)]
pub struct Tx {
    pub hash: String,
}

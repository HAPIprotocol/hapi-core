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
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone, Debug)]
pub struct Tx {
    pub hash: String,
}

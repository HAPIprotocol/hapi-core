use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("URL parse error: {0}")]
    UrlParseError(String),
    #[error("ETH address parse error: {0}")]
    EthAddressParse(String),
    #[error("Ethers error: {0}")]
    Ethers(String),
    #[error("Provider error: {0}")]
    Provider(#[from] ethers_providers::ProviderError),
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Default, Clone)]
pub struct Tx {
    pub hash: String,
}

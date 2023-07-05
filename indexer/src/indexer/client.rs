use {
    anyhow::Result,
    ethers::providers::{Http, Provider},
};

use super::Network;

#[derive(Debug)]
pub(crate) enum IndexerClient {
    Ethers(Provider<Http>),
    Near,
    Solana,
}

impl IndexerClient {
    pub fn new(network: Network, rpc_node_url: &str) -> Result<Self> {
        match network {
            Network::Ethereum => Ok(Self::Ethers(Provider::<Http>::try_from(rpc_node_url)?)),
            Network::Near => Ok(Self::Near),
            Network::Solana => Ok(Self::Solana),
            _ => Err(anyhow::anyhow!("Invalid network: {network:?}")),
        }
    }
}

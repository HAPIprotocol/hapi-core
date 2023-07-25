use {
    anyhow::Result,
    hapi_core::{HapiCoreEvm, HapiCoreNetwork, HapiCoreOptions},
};

pub(crate) enum IndexerClient {
    Evm(HapiCoreEvm),
    Near,
    Solana,
}

impl IndexerClient {
    pub fn new(
        network: HapiCoreNetwork,
        rpc_node_url: &str,
        contract_address: &str,
    ) -> Result<Self> {
        match network {
            HapiCoreNetwork::Ethereum | HapiCoreNetwork::Bsc | HapiCoreNetwork::Sepolia => {
                let cli = HapiCoreEvm::new(HapiCoreOptions {
                    provider_url: rpc_node_url.to_string(),
                    contract_address: contract_address.to_string(),
                    private_key: None,
                    chain_id: None,
                })?;

                Ok(Self::Evm(cli))
            }
            HapiCoreNetwork::Near => Ok(Self::Near),
            HapiCoreNetwork::Solana | HapiCoreNetwork::Bitcoin => Ok(Self::Solana),
        }
    }
}

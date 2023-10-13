use {
    anyhow::Result,
    ethers::types::Address,
    hapi_core::{HapiCoreEvm, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana},
};

use super::{
    evm::{process_evm_job_log, update_evm_block, update_evm_empty_cursor, update_evm_transaction},
    push::PushPayload,
    solana::process_solana_job,
    IndexerJob, IndexerState,
};

pub(crate) enum IndexerClient {
    Evm(HapiCoreEvm),
    Near,
    Solana(HapiCoreSolana),
}

impl IndexerClient {
    pub fn new(
        network: HapiCoreNetwork,
        rpc_node_url: &str,
        contract_address: &str,
    ) -> Result<Self> {
        let options = HapiCoreOptions {
            provider_url: rpc_node_url.to_string(),
            contract_address: contract_address.to_string(),
            private_key: None,
            chain_id: None,
            network: network.clone(),
        };

        match network {
            HapiCoreNetwork::Ethereum | HapiCoreNetwork::Bsc | HapiCoreNetwork::Sepolia => {
                Ok(Self::Evm(HapiCoreEvm::new(options)?))
            }
            HapiCoreNetwork::Near => Ok(Self::Near),
            HapiCoreNetwork::Solana | HapiCoreNetwork::Bitcoin => {
                Ok(Self::Solana(HapiCoreSolana::new(options)?))
            }
        }
    }

    pub(super) async fn handle_update_cursor(
        &self,
        contract_address: Address,
    ) -> Result<IndexerState> {
        match self {
            IndexerClient::Evm(client) => update_evm_empty_cursor(client, contract_address).await,
            _ => unimplemented!(),
        }
    }

    pub(super) async fn handle_update_block(
        &self,
        last_block: u64,
        contract_address: Address,
    ) -> Result<(IndexerState, Vec<IndexerJob>)> {
        match self {
            IndexerClient::Evm(client) => {
                update_evm_block(client, last_block, contract_address).await
            }
            _ => unimplemented!(),
        }
    }

    pub(super) async fn handle_update_transaction(&self, hash: String) -> Result<IndexerState> {
        match self {
            IndexerClient::Evm(client) => update_evm_transaction(client, hash).await,
            _ => unimplemented!(),
        }
    }

    pub(super) async fn handle_process(&self, job: IndexerJob) -> Result<Option<Vec<PushPayload>>> {
        match (self, job) {
            (IndexerClient::Evm(client), IndexerJob::Log(log)) => {
                process_evm_job_log(client, &log).await
            }
            (IndexerClient::Solana(client), IndexerJob::Transaction(hash)) => {
                process_solana_job(client, &hash).await
            }
            _ => unimplemented!(),
        }
    }
}

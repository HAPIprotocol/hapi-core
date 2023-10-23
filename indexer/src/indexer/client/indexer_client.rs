use {
    anyhow::Result,
    hapi_core::{HapiCoreEvm, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana},
};

use super::{
    evm::{fetch_evm_jobs, process_evm_job},
    solana::{fetch_solana_jobs, process_solana_job},
};

use crate::indexer::{push::PushPayload, IndexerJob, IndexingCursor};

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
            account_id: None,
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

    pub(crate) async fn fetch_jobs(&self, cursor: &IndexingCursor) -> Result<Vec<IndexerJob>> {
        match (self, cursor) {
            (IndexerClient::Evm(client), IndexingCursor::Block(n)) => {
                fetch_evm_jobs(client, Some(*n)).await
            }
            (IndexerClient::Evm(client), IndexingCursor::None) => {
                fetch_evm_jobs(client, None).await
            }

            (IndexerClient::Solana(client), IndexingCursor::Transaction(tx)) => {
                fetch_solana_jobs(client, Some(tx)).await
            }
            (IndexerClient::Solana(client), IndexingCursor::None) => {
                fetch_solana_jobs(client, None).await
            }
            _ => unimplemented!(),
        }
    }

    pub(crate) async fn handle_process(&self, job: IndexerJob) -> Result<Option<Vec<PushPayload>>> {
        match (self, job) {
            (IndexerClient::Evm(client), IndexerJob::Log(log)) => {
                process_evm_job(client, &log).await
            }
            (IndexerClient::Solana(client), IndexerJob::Transaction(hash)) => {
                process_solana_job(client, &hash).await
            }
            _ => unimplemented!(),
        }
    }
}

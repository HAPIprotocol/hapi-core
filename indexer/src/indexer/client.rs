use {
    anyhow::Result,
    hapi_core::{HapiCoreEvm, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana},
};

use super::{
    evm::{process_evm_job_log, update_evm_cursor},
    push::PushPayload,
    solana::{process_solana_job, update_solana_cursor},
    IndexerJob, IndexingCursor,
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

    pub(super) async fn handle_update(&self, cursor: &IndexingCursor) -> Result<Vec<IndexerJob>> {
        match (self, cursor) {
            (IndexerClient::Evm(client), IndexingCursor::Block(n)) => {
                update_evm_cursor(client, Some(n.clone())).await
            }
            (IndexerClient::Evm(client), IndexingCursor::None) => {
                update_evm_cursor(client, None).await
            }

            (IndexerClient::Solana(client), IndexingCursor::Transaction(tx)) => {
                update_solana_cursor(client, Some(&tx)).await
            }
            (IndexerClient::Solana(client), IndexingCursor::None) => {
                update_solana_cursor(client, None).await
            }
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

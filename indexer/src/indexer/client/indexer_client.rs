use {
    anyhow::Result,
    hapi_core::{HapiCoreEvm, HapiCoreNear, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana},
    std::time::Duration,
};

use super::{
    evm::{fetch_evm_jobs, process_evm_job},
    near::{fetch_near_jobs, process_near_job},
    solana::{fetch_solana_jobs, process_solana_job},
};

use crate::indexer::{push::PushPayload, IndexerJob, IndexingCursor};

pub const DEFAULT_PAGE_SIZE: u64 = 600;
lazy_static::lazy_static! {
    pub static ref PAGE_SIZE: u64 = std::env::var("INDEXER_PAGE_SIZE").map_or(DEFAULT_PAGE_SIZE, |s| s.parse::<u64>().unwrap_or(DEFAULT_PAGE_SIZE));
}

pub(crate) enum IndexerClient {
    Evm(HapiCoreEvm),
    Near(HapiCoreNear),
    Solana(HapiCoreSolana),
}

pub(crate) struct FetchingArtifacts {
    pub jobs: Vec<IndexerJob>,
    pub cursor: IndexingCursor,
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
            HapiCoreNetwork::Near => Ok(Self::Near(HapiCoreNear::new(options)?)),
            HapiCoreNetwork::Solana | HapiCoreNetwork::Bitcoin => {
                Ok(Self::Solana(HapiCoreSolana::new(options)?))
            }
        }
    }

    pub(crate) async fn fetch_jobs(
        &self,
        cursor: &IndexingCursor,
        fetching_delay: Duration,
    ) -> Result<FetchingArtifacts> {
        match self {
            IndexerClient::Evm(client) => fetch_evm_jobs(client, cursor, fetching_delay).await,
            IndexerClient::Solana(client) => {
                fetch_solana_jobs(client, cursor, fetching_delay).await
            }
            IndexerClient::Near(client) => fetch_near_jobs(client, cursor, fetching_delay).await,
        }
    }

    pub(crate) async fn handle_process(
        &self,
        job: &IndexerJob,
    ) -> Result<Option<Vec<PushPayload>>> {
        match (self, job) {
            (IndexerClient::Evm(client), IndexerJob::Log(log)) => {
                process_evm_job(client, log).await
            }
            (IndexerClient::Solana(client), IndexerJob::Transaction(hash)) => {
                process_solana_job(client, hash).await
            }
            (IndexerClient::Near(client), IndexerJob::TransactionReceipt(receipt)) => {
                process_near_job(client, receipt).await
            }
            _ => unimplemented!(),
        }
    }
}

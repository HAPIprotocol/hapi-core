use {
    anyhow::Result,
    hapi_core::{HapiCoreEvm, HapiCoreNear, HapiCoreNetwork, HapiCoreOptions, HapiCoreSolana},
    std::time::Duration,
    tokio::time::sleep,
    uuid::Uuid,
};

use super::{
    evm::{fetch_evm_jobs, process_evm_job},
    near::{fetch_near_jobs, process_near_job},
    solana::{fetch_solana_jobs, process_solana_job},
};

use crate::indexer::{push::PushPayload, IndexerJob, IndexingCursor};

pub const DEFAULT_PAGE_SIZE: u64 = 500;
lazy_static::lazy_static! {
    pub static ref PAGE_SIZE: u64 = std::env::var("INDEXER_PAGE_SIZE").map_or(DEFAULT_PAGE_SIZE, |s| s.parse::<u64>().unwrap_or(DEFAULT_PAGE_SIZE));
}

pub(crate) enum HapiClient {
    Evm(HapiCoreEvm),
    Near(HapiCoreNear),
    Solana(HapiCoreSolana),
}

pub(crate) struct FetchingArtifacts {
    pub jobs: Vec<IndexerJob>,
    pub cursor: IndexingCursor,
}

pub(crate) struct IndexerClient {
    id: Uuid,
    client: HapiClient,
    network: HapiCoreNetwork,
    fetching_delay: Duration,
}

impl IndexerClient {
    pub fn new(
        id: Uuid,
        network: HapiCoreNetwork,
        rpc_node_url: &str,
        contract_address: &str,
        fetching_delay: Duration,
    ) -> Result<Self> {
        let options = HapiCoreOptions {
            provider_url: rpc_node_url.to_string(),
            contract_address: contract_address.to_string(),
            private_key: None,
            chain_id: None,
            account_id: None,
            network: network.clone(),
        };

        let client = match network {
            HapiCoreNetwork::Ethereum | HapiCoreNetwork::Bsc | HapiCoreNetwork::Sepolia => {
                HapiClient::Evm(HapiCoreEvm::new(options)?)
            }
            HapiCoreNetwork::Near => HapiClient::Near(HapiCoreNear::new(options)?),
            HapiCoreNetwork::Solana | HapiCoreNetwork::Bitcoin => {
                HapiClient::Solana(HapiCoreSolana::new(options)?)
            }
        };

        Ok(Self {
            id,
            client,
            network,
            fetching_delay,
        })
    }

    pub(crate) async fn fetch_jobs(&self, cursor: &IndexingCursor) -> Result<FetchingArtifacts> {
        let artifacts = match &self.client {
            HapiClient::Evm(client) => fetch_evm_jobs(client, cursor).await?,
            HapiClient::Solana(client) => {
                fetch_solana_jobs(client, cursor, self.fetching_delay).await?
            }
            HapiClient::Near(client) => fetch_near_jobs(client, cursor).await?,
        };

        sleep(self.fetching_delay).await;

        Ok(artifacts)
    }

    pub(crate) async fn handle_process(
        &self,
        job: &IndexerJob,
    ) -> Result<Option<Vec<PushPayload>>> {
        match (&self.client, job) {
            (HapiClient::Evm(client), IndexerJob::Log(log)) => {
                process_evm_job(client, log, &self.network, self.id).await
            }
            (HapiClient::Solana(client), IndexerJob::Transaction(hash)) => {
                process_solana_job(client, hash, &self.network, self.id).await
            }
            (HapiClient::Near(client), IndexerJob::TransactionReceipt(receipt)) => {
                process_near_job(client, receipt, &self.network, self.id).await
            }
            _ => unimplemented!(),
        }
    }

    pub(crate) fn get_id(&self) -> Uuid {
        self.id
    }
}

use {
    ethers::types::Log,
    serde::{Deserialize, Serialize},
};

use hapi_core::HapiCoreEvm;

use super::{Indexer, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum IndexerJob {
    Transaction(String),
    Log(Log),
}

impl Indexer {
    pub(super) async fn process_evm_job_log(
        &self,
        client: &HapiCoreEvm,
        log: &ethers::types::Log,
    ) -> Result<()> {
        let log_header = client.decode_event(log)?;

        tracing::info!(
            name = log_header.name,
            tx = ?log.transaction_hash,
            block = ?log.block_number,
            tokens = ?log_header.tokens,
            "Found event",
        );

        match log_header.to_ref() {
            (
                "ReporterCreated"
                | "ReporterUpdated"
                | "ReporterActivated"
                | "ReporterDeactivated"
                | "ReporterStakeWithdrawn",
                [reporter_id, ..],
            ) => {
                tracing::info!(?reporter_id, "Reporter is created or modified");
            }
            ("CaseCreated" | "CaseUpdated", [case_id, ..]) => {
                tracing::info!(?case_id, "Case is created or modified");
            }
            ("AddressCreated" | "AddressUpdated", [addr, ..]) => {
                tracing::info!(?addr, "Address is created or modified");
            }
            ("AssetCreated" | "AssetUpdated", [addr, id, ..]) => {
                tracing::info!(?addr, ?id, "Asset is created or modified");
            }
            (
                "AuthorityChanged" | "StakeConfigurationChanged" | "RewardConfigurationChanged",
                [..],
            ) => {
                tracing::info!("Configuration is changed");
            }
            _ => {
                tracing::warn!(name = log_header.name, tokens = ?log_header.tokens, "Unable to process event")
            }
        };

        Ok(())
    }
}

use uuid::Uuid;
use {
    ethers::{providers::Middleware, types::Log},
    serde::{Deserialize, Serialize},
};

use hapi_core::{HapiCore, HapiCoreEvm};

use super::{
    push::{PushEvent, PushEventName, PushPayload},
    Indexer, Result,
};

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

        let tx = format!(
            "{:#?}",
            log.transaction_hash
                .ok_or_else(|| anyhow::anyhow!("Unable to parse transaction hash"))?
        );

        let block_number = log
            .block_number
            .ok_or_else(|| anyhow::anyhow!("Unable to parse block number"))?
            .as_u64();

        tracing::info!(
            name = log_header.name,
            tx,
            block = block_number,
            tokens = ?log_header.tokens,
            "Found event",
        );

        match log_header.to_ref() {
            v @ (
                "ReporterCreated"
                | "ReporterUpdated"
                | "ReporterActivated"
                | "ReporterDeactivated"
                | "ReporterStakeWithdrawn",
                [reporter_id, ..],
            ) => {
                if let Some(reporter_id) = reporter_id.clone().into_uint() {
                    let reporter_id = Uuid::from_u128(reporter_id.as_u128());
                    tracing::info!(?reporter_id, "Reporter is created or modified");

                    let block = client
                        .provider
                        .get_block(block_number)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Unable to get block"))?;

                    let reporter = client.get_reporter(&reporter_id.to_string()).await?;

                    self.send_webhook(&PushPayload {
                        event: PushEvent {
                            name: match v.0 {
                                "ReporterCreated" => PushEventName::CreateReporter,
                                "ReporterUpdated" => PushEventName::UpdateReporter,
                                "ReporterActivated" => PushEventName::ActivateReporter,
                                "ReporterDeactivated" => PushEventName::DeactivateReporter,
                                "ReporterStakeWithdrawn" => PushEventName::Unstake,
                                _ => unreachable!(),
                            },
                            tx_hash: tx,
                            tx_index: 0,
                            timestamp: block.timestamp.as_u64(),
                        },
                        data: reporter.into(),
                    })
                    .await?;
                } else {
                    tracing::warn!(?reporter_id, "Unable to parse reporter id");
                }
            }
            v @ ("CaseCreated" | "CaseUpdated", [case_id, ..]) => {
                if let Some(case_id) = case_id.clone().into_uint() {
                    let case_id = Uuid::from_u128(case_id.as_u128());
                    tracing::info!(?case_id, "Case is created or modified");

                    let block = client
                        .provider
                        .get_block(block_number)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Unable to get block"))?;

                    let case = client.get_case(&case_id.to_string()).await?;

                    self.send_webhook(&PushPayload {
                        event: PushEvent {
                            name: match v.0 {
                                "CaseCreated" => PushEventName::CreateCase,
                                "CaseUpdated" => PushEventName::UpdateCase,
                                _ => unreachable!(),
                            },
                            tx_hash: tx,
                            tx_index: 0,
                            timestamp: block.timestamp.as_u64(),
                        },
                        data: case.into(),
                    })
                    .await?;
                } else {
                    tracing::warn!(?case_id, "Unable to parse case id");
                }
            }
            v @ ("AddressCreated" | "AddressUpdated", [addr, ..]) => {
                if let Some(addr) = addr.clone().into_address() {
                    tracing::info!(?addr, "Address is created or modified");

                    let block = client
                        .provider
                        .get_block(block_number)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Unable to get block"))?;

                    let address = client.get_address(&format!("{addr:?}")).await?;

                    self.send_webhook(&PushPayload {
                        event: PushEvent {
                            name: match v.0 {
                                "AddressCreated" => PushEventName::CreateAddress,
                                "AddressUpdated" => PushEventName::UpdateAddress,
                                _ => unreachable!(),
                            },
                            tx_hash: tx,
                            tx_index: 0,
                            timestamp: block.timestamp.as_u64(),
                        },
                        data: address.into(),
                    })
                    .await?;
                } else {
                    tracing::warn!(?addr, "Unable to parse address");
                }
            }
            v @ ("AssetCreated" | "AssetUpdated", [addr, id, ..]) => {
                if let (Some(addr), Some(id)) =
                    (addr.clone().into_address(), id.clone().into_uint())
                {
                    tracing::info!(?addr, ?id, "Asset is created or modified");

                    let block = client
                        .provider
                        .get_block(block_number)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Unable to get block"))?;

                    let asset = client.get_asset(&format!("{addr:?}"), &id.into()).await?;

                    self.send_webhook(&PushPayload {
                        event: PushEvent {
                            name: match v.0 {
                                "AssetCreated" => PushEventName::CreateAsset,
                                "AssetUpdated" => PushEventName::UpdateAsset,
                                _ => unreachable!(),
                            },
                            tx_hash: tx,
                            tx_index: 0,
                            timestamp: block.timestamp.as_u64(),
                        },
                        data: asset.into(),
                    })
                    .await?;
                } else {
                    tracing::warn!(?addr, ?id, "Unable to parse asset");
                }
            }
            (
                "AuthorityChanged" | "StakeConfigurationChanged" | "RewardConfigurationChanged",
                [..],
            ) => {
                tracing::info!("Configuration is changed");
            }
            _ => {
                tracing::warn!(name = log_header.name, tokens = ?log_header.tokens, "Ignoring event")
            }
        };

        Ok(())
    }
}

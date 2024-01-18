use {
    anyhow::{bail, Result},
    hapi_core::{
        client::{entities::asset::AssetId, events::EventName},
        HapiCore, HapiCoreNear, HapiCoreNetwork,
    },
    near_jsonrpc_client::methods::{
        EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest,
        EXPERIMENTAL_receipt::RpcReceiptRequest,
    },
    near_jsonrpc_primitives::types::receipts::ReceiptReference,
    near_primitives::{
        hash::CryptoHash,
        types::{BlockId, BlockReference, Finality, FunctionArgs, StoreKey},
        views::{
            ActionView, ReceiptEnumView, ReceiptView, StateChangeCauseView, StateChangesRequestView,
        },
    },
    std::{cmp::min, collections::HashSet},
    uuid::Uuid,
};

use crate::{
    indexer::{
        client::indexer_client::PAGE_SIZE,
        push::{PushEvent, PushPayload},
        IndexerJob,
    },
    IndexingCursor,
};

use super::indexer_client::FetchingArtifacts;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NearReceipt {
    pub hash: CryptoHash,
    pub block_height: u64,
    pub timestamp: u64,
}

async fn get_receipts_list(
    client: &HapiCoreNear,
    start_block: u64,
    final_block: u64,
) -> Result<Vec<IndexerJob>> {
    let mut event_list = vec![];

    for block_height in start_block..final_block + 1 {
        let block_id = BlockId::Height(block_height);

        let changes_in_block = client
            .client
            .call(RpcStateChangesInBlockByTypeRequest {
                block_reference: BlockReference::BlockId(block_id.clone()),
                state_changes_request: StateChangesRequestView::DataChanges {
                    account_ids: vec![client.contract_address.clone()],
                    key_prefix: StoreKey::from(vec![]),
                },
            })
            .await;

        match changes_in_block {
            Ok(changes) => {
                if !changes.changes.is_empty() {
                    let timestamp = client
                        .client
                        .call(near_jsonrpc_primitives::types::blocks::RpcBlockRequest {
                            block_reference: BlockReference::BlockId(block_id),
                        })
                        .await?
                        .header
                        .timestamp_nanosec;

                    changes
                        .changes
                        .iter()
                        .map(|change| get_hash_from_cause(&change.cause))
                        .collect::<HashSet<CryptoHash>>()
                        .iter()
                        .for_each(|&hash| {
                            event_list.push(IndexerJob::TransactionReceipt(NearReceipt {
                                hash,
                                block_height,
                                timestamp,
                            }));
                        })
                }
            }
            Err(e) => {
                tracing::error!(block_height, "Failed to fetch near jobs: {:?}", e);
            }
        };
    }

    Ok(event_list)
}

#[tracing::instrument(skip(client))]
pub(super) async fn fetch_near_jobs(
    client: &HapiCoreNear,
    current_cursor: &IndexingCursor,
) -> Result<FetchingArtifacts> {
    let start_block = match current_cursor {
        IndexingCursor::None => 0,
        IndexingCursor::Block(block) => *block + 1,
        _ => bail!("Near network must have a block cursor"),
    };

    let latest_block = client
        .client
        .call(near_jsonrpc_primitives::types::blocks::RpcBlockRequest {
            block_reference: BlockReference::Finality(Finality::Final),
        })
        .await?
        .header
        .height;

    if start_block < latest_block {
        tracing::info!(start_block, "Fetching near jobs from");

        let final_block = min(PAGE_SIZE.to_owned() - 1 + start_block, latest_block);

        let event_list: Vec<IndexerJob> =
            get_receipts_list(client, start_block, final_block).await?;

        tracing::info!(count = event_list.len(), "Found jobs");

        return Ok(FetchingArtifacts {
            jobs: event_list,
            cursor: IndexingCursor::Block(final_block),
        });
    }

    tracing::trace!("No new blocks found");

    Ok(FetchingArtifacts {
        jobs: vec![],
        cursor: current_cursor.clone(),
    })
}

#[tracing::instrument(skip(client, network), fields(receipt_hash = %receipt.hash))]
pub(super) async fn process_near_job(
    client: &HapiCoreNear,
    receipt: &NearReceipt,
    network: &HapiCoreNetwork,
    id: Uuid,
) -> Result<Option<Vec<PushPayload>>> {
    let receipt_view = client
        .client
        .call(RpcReceiptRequest {
            receipt_reference: ReceiptReference {
                receipt_id: receipt.hash,
            },
        })
        .await?;

    if let Some((method, args)) = get_method_from_receipt(&receipt_view) {
        let event_name: EventName = {
            if method == "ft_on_transfer" {
                // because activation in NEAR is done by ft_transfer_call
                EventName::ActivateReporter
            } else {
                match method.parse() {
                    Ok(event_name) => event_name,
                    Err(e) => {
                        tracing::error!(method, "Failed to parse method {}: {:?}", method, e);
                        return Ok(None);
                    }
                }
            }
        };

        let data = match event_name {
            EventName::CreateReporter
            | EventName::UpdateReporter
            | EventName::DeactivateReporter
            | EventName::Unstake => {
                tracing::info!("Reporter updated");

                let id = get_id_from_args(&args).await?;
                client.get_reporter(&id.to_string()).await?.into()
            }
            EventName::ActivateReporter => {
                tracing::info!("Reporter activated");

                let account_id = get_field_from_args(&args, "sender_id")?;
                client.get_reporter_by_account(&account_id).await?.into()
            }
            EventName::CreateCase | EventName::UpdateCase => {
                tracing::info!("Case is created or modified");

                let id = get_id_from_args(&args).await?;
                client.get_case(&id.to_string()).await?.into()
            }
            EventName::CreateAddress | EventName::UpdateAddress => {
                tracing::info!("Address is created or modified");

                let address = get_field_from_args(&args, "address")?;
                client.get_address(&address).await?.into()
            }
            EventName::ConfirmAddress | EventName::ConfirmAsset => {
                tracing::info!("Confirmation is received");
                return Ok(None);
            }
            EventName::CreateAsset | EventName::UpdateAsset => {
                tracing::info!("Asset is created or modified");
                let addr = get_field_from_args(&args, "address")?;
                let asset_id = get_field_from_args(&args, "id")?;
                client
                    .get_asset(&addr, &asset_id.parse::<AssetId>()?)
                    .await?
                    .into()
            }

            EventName::UpdateStakeConfiguration
            | EventName::UpdateRewardConfiguration
            | EventName::SetAuthority => {
                tracing::info!("Configuration is changed");
                return Ok(None);
            }
            EventName::Initialize => {
                tracing::info!("Contract initialized");
                return Ok(None);
            }
        };

        return Ok(Some(vec![PushPayload {
            id,
            network: network.clone(),
            event: PushEvent {
                name: event_name,
                tx_hash: receipt.hash.to_string(),
                tx_index: 0,
                timestamp: receipt.timestamp,
            },
            data,
        }]));
    }
    Ok(None)
}

fn get_hash_from_cause(cause: &StateChangeCauseView) -> CryptoHash {
    match cause {
        StateChangeCauseView::TransactionProcessing { tx_hash } => *tx_hash,
        StateChangeCauseView::ReceiptProcessing { receipt_hash } => *receipt_hash,
        _ => CryptoHash::default(),
    }
}

fn get_method_from_receipt(receipt: &ReceiptView) -> Option<(String, FunctionArgs)> {
    match &receipt.receipt {
        ReceiptEnumView::Action {
            signer_id: _,
            signer_public_key: _,
            gas_price: _,
            output_data_receivers: _,
            input_data_ids: _,
            actions,
        } => match &actions[0] {
            ActionView::FunctionCall {
                method_name,
                args,
                gas: _,
                deposit: _,
            } => Some((method_name.clone(), args.clone())),
            _ => None,
        },
        _ => None,
    }
}

fn get_field_from_args(args: &FunctionArgs, field: &str) -> Result<String> {
    let json: serde_json::Value = serde_json::from_slice(args)?;

    if let Some(value) = json[field].as_str() {
        Ok(value.to_string())
    } else {
        Err(anyhow::anyhow!("Failed to parse {} from {:?}", field, json))
    }
}

async fn get_id_from_args(args: &FunctionArgs) -> Result<Uuid> {
    let json: serde_json::Value = serde_json::from_slice(args)?;

    if let Some(id) = json["id"].as_str() {
        Ok(Uuid::from_u128(id.parse::<u128>()?))
    } else {
        Err(anyhow::anyhow!("Failed to parse id from {:?}", json))
    }
}

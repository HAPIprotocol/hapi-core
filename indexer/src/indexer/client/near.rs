use {
    anyhow::Result,
    hapi_core::{client::events::EventName, HapiCore, HapiCoreNear},
    near_jsonrpc_client::{
        errors::{JsonRpcError, JsonRpcServerError},
        methods::{
            EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest,
            EXPERIMENTAL_receipt::RpcReceiptRequest,
        },
    },
    near_jsonrpc_primitives::types::{changes::RpcStateChangesError, receipts::ReceiptReference},
    near_primitives::{
        hash::CryptoHash,
        types::{BlockId, BlockReference, FunctionArgs, StoreKey},
        views::{
            ActionView, ReceiptEnumView, ReceiptView, StateChangeCauseView, StateChangesRequestView,
        },
    },
    std::collections::HashSet,
    tokio::time::sleep,
    uuid::Uuid,
};

use hapi_core::client::entities::asset::AssetId;

use crate::{
    indexer::{
        push::{PushEvent, PushPayload},
        IndexerJob,
    },
    IndexingCursor, ITERATION_INTERVAL,
};

use super::indexer_client::FetchingArtifacts;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NearReceipt {
    pub hash: CryptoHash,
    pub block_height: u64,
    pub timestamp: u64,
}

const NEAR_PAGE_SIZE: u64 = 600;

pub(super) async fn fetch_near_jobs(
    client: &HapiCoreNear,
    current_cursor: Option<u64>,
) -> Result<FetchingArtifacts> {
    let start_block_height = current_cursor.unwrap_or_default();
    let mut event_list = vec![];

    let mut block_height = start_block_height;
    let mut block_not_found_flag = false;

    tracing::info!(block_height, "Fetching near jobs");

    loop {
        let start_timestamp = std::time::Instant::now();

        if block_height - start_block_height >= NEAR_PAGE_SIZE {
            break;
        };
        block_height += 1;

        let contract_address = client.contract_address.clone();
        let client = client.client.clone();
        let block_id = BlockId::Height(block_height);

        let changes_in_block = client
            .call(RpcStateChangesInBlockByTypeRequest {
                block_reference: BlockReference::BlockId(block_id.clone()),
                state_changes_request: StateChangesRequestView::DataChanges {
                    account_ids: vec![contract_address],
                    key_prefix: StoreKey::from(vec![]),
                },
            })
            .await;

        let changes = match changes_in_block {
            Ok(changes) => {
                block_not_found_flag = false;
                changes.changes
            }
            Err(e) => {
                match e {
                    JsonRpcError::ServerError(JsonRpcServerError::HandlerError(
                        RpcStateChangesError::UnknownBlock { .. },
                    )) => {}
                    _ => {
                        tracing::error!(block_height, "Failed to fetch changes for block");
                    }
                }
                if block_not_found_flag {
                    block_height -= 2;
                    break;
                }
                block_not_found_flag = true;
                continue;
            }
        };

        if !changes.is_empty() {
            let hashes: HashSet<CryptoHash> = changes
                .iter()
                .map(|change| get_hash_from_cause(&change.cause).expect("no hash"))
                .collect();

            let timestamp = client
                .call(near_jsonrpc_primitives::types::blocks::RpcBlockRequest {
                    block_reference: BlockReference::BlockId(block_id),
                })
                .await?
                .header
                .timestamp_nanosec;

            for hash in hashes {
                event_list.push(IndexerJob::TransactionReceipt(NearReceipt {
                    hash,
                    block_height,
                    timestamp,
                }));
            }
        }

        let time_passed = start_timestamp.elapsed();
        if time_passed < ITERATION_INTERVAL {
            sleep(ITERATION_INTERVAL - time_passed).await;
        }
    }
    tracing::info!(block_height, "Fetched until block {}", block_height);

    Ok(FetchingArtifacts {
        jobs: event_list,
        cursor: IndexingCursor::Block(block_height),
    })
}

pub(super) async fn process_near_job(
    client: &HapiCoreNear,
    receipt: &NearReceipt,
) -> Result<Option<Vec<PushPayload>>> {
    tracing::info!("Process near jobs");

    let receipt_view = client
        .client
        .call(RpcReceiptRequest {
            receipt_reference: ReceiptReference {
                receipt_id: receipt.hash,
            },
        })
        .await?;

    let (method, args) =
        get_method_from_receipt(&receipt_view).expect("Err get method_from_receipt");

    let event_name: EventName = {
        if method == "ft_on_transfer" {
            // because activation in NEAR is done by ft_transfer_call
            EventName::ActivateReporter
        } else {
            method
                .parse()
                .expect(format!("Failed to parse method {} tx {}", method, receipt.hash).as_str())
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

            let account_id = get_field_from_args(&args, "sender_id");
            client.get_reporter_by_account(&account_id).await?.into()
        }
        EventName::CreateCase | EventName::UpdateCase => {
            tracing::info!("Case updated");

            let id = get_id_from_args(&args).await?;
            client.get_case(&id.to_string()).await?.into()
        }
        EventName::CreateAddress | EventName::UpdateAddress => {
            tracing::info!("Address updated");

            let address = get_field_from_args(&args, "address");
            client.get_address(&address).await?.into()
        }
        EventName::ConfirmAddress | EventName::ConfirmAsset => {
            tracing::info!("Confirmation is received");
            return Ok(None);
        }
        EventName::CreateAsset | EventName::UpdateAsset => {
            tracing::info!("Asset updated");
            let addr = get_field_from_args(&args, "address");
            let asset_id = get_field_from_args(&args, "id").parse::<AssetId>()?;
            client.get_asset(&addr, &asset_id).await?.into()
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

    let payload = PushPayload {
        event: PushEvent {
            name: event_name,
            tx_hash: receipt.hash.to_string(),
            tx_index: 0,
            timestamp: receipt.timestamp,
        },
        data,
    };
    Ok(Some(vec![payload]))
}

fn get_hash_from_cause(cause: &StateChangeCauseView) -> Option<CryptoHash> {
    match cause {
        StateChangeCauseView::TransactionProcessing { tx_hash } => Some(*tx_hash),
        StateChangeCauseView::ReceiptProcessing { receipt_hash } => Some(*receipt_hash),
        _ => None,
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

fn get_field_from_args(args: &FunctionArgs, field: &str) -> String {
    let json: serde_json::Value = serde_json::from_slice(args).expect("Failed to parse args");

    json[field]
        .as_str()
        .expect(format!("Failed to parse {} from {:?}", field, json).as_str())
        .to_string()
}

async fn get_id_from_args(args: &FunctionArgs) -> Result<Uuid> {
    let json: serde_json::Value = serde_json::from_slice(args)?;

    let id = json["id"]
        .as_str()
        .expect(format!("Failed to parse id from {:?}", json).as_str());
    Ok(Uuid::from_u128(id.parse::<u128>()?))
}

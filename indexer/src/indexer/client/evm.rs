use {
    anyhow::Result,
    ethers::{abi::Token, providers::Middleware, types::Filter},
    hapi_core::{client::events::EventName, HapiCore, HapiCoreEvm},
    std::str::FromStr,
    uuid::Uuid,
};

use crate::indexer::{
    push::{PushData, PushEvent, PushPayload},
    IndexerJob,
};

pub const EVM_PAGE_SIZE: u64 = 100;

pub(super) async fn fetch_evm_jobs(
    client: &HapiCoreEvm,
    current_cursor: Option<u64>,
) -> Result<Vec<IndexerJob>> {
    let filter = Filter::default().address(client.contract.address());
    let mut earliest_block = current_cursor.unwrap_or_default();
    let mut event_list = vec![];

    tracing::info!(earliest_block, "Fetching evm jobs");

    loop {
        let next_block = earliest_block + EVM_PAGE_SIZE;

        let logs = client
            .contract
            .client()
            .get_logs(
                &filter
                    .clone()
                    .from_block(earliest_block)
                    .to_block(next_block),
            )
            .await
            .expect("Failed to fetch logs");

        if logs.is_empty() {
            break;
        }

        logs.into_iter().for_each(|log| {
            event_list.push(IndexerJob::Log(log));
        });

        earliest_block = next_block;
    }

    return Ok(event_list);
}

pub(super) async fn process_evm_job(
    client: &HapiCoreEvm,
    log: &ethers::types::Log,
) -> Result<Option<Vec<PushPayload>>> {
    let log_header = client.decode_event(log)?;

    let tx_hash = format!(
        "{:#?}",
        log.transaction_hash
            .ok_or_else(|| anyhow::anyhow!("Unable to parse transaction hash"))?
    );

    let block_number = log
        .block_number
        .ok_or_else(|| anyhow::anyhow!("Unable to parse block number"))?
        .as_u64();

    let block = client
        .provider
        .get_block(block_number)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Unable to get block"))?;

    tracing::info!(
        name = log_header.name,
        tx_hash,
        block = block_number,
        tokens = ?log_header.tokens,
        "Found event",
    );

    let data = match log_header.to_ref() {
        (
            "ReporterCreated"
            | "ReporterUpdated"
            | "ReporterActivated"
            | "ReporterDeactivated"
            | "ReporterStakeWithdrawn",
            [reporter_id, ..],
        ) => get_evm_reporter_payload(client, reporter_id).await?,
        ("CaseCreated" | "CaseUpdated", [case_id, ..]) => {
            get_evm_case_payload(client, case_id).await?
        }
        ("AddressCreated" | "AddressUpdated", [addr, ..]) => {
            get_evm_address_payload(client, addr).await?
        }
        ("AssetCreated" | "AssetUpdated", [addr, id, ..]) => {
            get_evm_asset_payload(client, addr, id).await?
        }
        ("AuthorityChanged" | "StakeConfigurationChanged" | "RewardConfigurationChanged", [..]) => {
            tracing::info!("Configuration is changed");
            None
        }
        _ => {
            tracing::warn!(name = log_header.name, tokens = ?log_header.tokens, "Ignoring event");
            None
        }
    };

    if let Some(data) = data {
        Ok(Some(vec![PushPayload {
            event: PushEvent {
                name: EventName::from_str(&log_header.name)?,
                tx_hash,
                tx_index: 0,
                timestamp: block.timestamp.as_u64(),
            },
            data,
        }]))
    } else {
        return Ok(None);
    }
}

async fn get_evm_reporter_payload(
    client: &HapiCoreEvm,
    reporter_id: &Token,
) -> Result<Option<PushData>> {
    if let Some(reporter_id) = reporter_id.clone().into_uint() {
        let reporter_id = Uuid::from_u128(reporter_id.as_u128());
        tracing::info!(?reporter_id, "Reporter is created or modified");

        let reporter = client.get_reporter(&reporter_id.to_string()).await?;

        Ok(Some(reporter.into()))
    } else {
        tracing::warn!(?reporter_id, "Unable to parse reporter id");
        Ok(None)
    }
}

async fn get_evm_case_payload(client: &HapiCoreEvm, case_id: &Token) -> Result<Option<PushData>> {
    if let Some(case_id) = case_id.clone().into_uint() {
        let case_id = Uuid::from_u128(case_id.as_u128());
        tracing::info!(?case_id, "Case is created or modified");

        let case = client.get_case(&case_id.to_string()).await?;

        Ok(Some(case.into()))
    } else {
        tracing::warn!(?case_id, "Unable to parse case id");
        Ok(None)
    }
}

async fn get_evm_address_payload(client: &HapiCoreEvm, addr: &Token) -> Result<Option<PushData>> {
    if let Some(addr) = addr.clone().into_address() {
        tracing::info!(?addr, "Address is created or modified");

        let address = client.get_address(&format!("{addr:?}")).await?;

        Ok(Some(address.into()))
    } else {
        tracing::warn!(?addr, "Unable to parse address");
        Ok(None)
    }
}

async fn get_evm_asset_payload(
    client: &HapiCoreEvm,
    addr: &Token,
    id: &Token,
) -> Result<Option<PushData>> {
    if let (Some(addr), Some(id)) = (addr.clone().into_address(), id.clone().into_uint()) {
        tracing::info!(?addr, ?id, "Asset is created or modified");

        let asset = client.get_asset(&format!("{addr:?}"), &id.into()).await?;

        Ok(Some(asset.into()))
    } else {
        tracing::warn!(?addr, ?id, "Unable to parse asset");
        Ok(None)
    }
}

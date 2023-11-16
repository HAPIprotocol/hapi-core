use {
    anyhow::{bail, Result},
    ethers::{abi::Token, providers::Middleware, types::Filter},
    hapi_core::{client::events::EventName, HapiCore, HapiCoreEvm},
    std::{cmp::min, str::FromStr},
    tokio::time::sleep,
    uuid::Uuid,
};

use crate::{
    indexer::{
        client::indexer_client::{ITERATION_INTERVAL, PAGE_SIZE},
        push::{PushData, PushEvent, PushPayload},
        IndexerJob,
    },
    IndexingCursor,
};

pub(super) async fn fetch_evm_jobs(
    client: &HapiCoreEvm,
    current_cursor: &IndexingCursor,
) -> Result<(Vec<IndexerJob>, IndexingCursor)> {
    let from_block = match &current_cursor {
        IndexingCursor::None => 0,
        IndexingCursor::Block(block) => *block + 1,
        _ => bail!("Evm network must have a block cursor"),
    };

    tracing::info!(from_block, "Fetching evm jobs from");

    let latest_block = client.provider.get_block_number().await?.as_u64();
    let event_list = get_event_list(client, min(from_block, latest_block), latest_block).await?;
    tracing::info!(count = event_list.len(), "Found jobs");

    let new_cursor = if let Some(recent) = event_list.first() {
        IndexingCursor::try_from(recent.clone())?
    } else {
        IndexingCursor::Block(latest_block)
    };

    Ok((event_list, new_cursor))
}

async fn get_event_list(
    client: &HapiCoreEvm,
    earliest_block: u64,
    latest_block: u64,
) -> Result<Vec<IndexerJob>> {
    let mut from_block = earliest_block;
    let mut event_list = vec![];
    let filter = Filter::default().address(client.contract.address());
    let size = PAGE_SIZE.saturating_sub(1);

    loop {
        let to_block = min(size.saturating_add(from_block), latest_block);

        let logs = client
            .contract
            .client()
            .get_logs(&filter.clone().from_block(from_block).to_block(to_block))
            .await
            .expect("Failed to fetch logs");

        logs.into_iter().for_each(|log| {
            event_list.push(IndexerJob::Log(log));
        });

        if to_block == latest_block {
            break;
        }

        from_block = to_block + 1;
        sleep(ITERATION_INTERVAL).await;
    }

    Ok(event_list)
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
        "Processing event",
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
        Ok(None)
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

use {
    anyhow::Result,
    ethers::{
        abi::Token,
        providers::Middleware,
        types::{Address, BlockNumber, Filter, H256},
    },
    hapi_core::{client::events::EventName, HapiCore, HapiCoreEvm},
    std::str::FromStr,
    uuid::Uuid,
};

use super::{
    now,
    push::{PushData, PushEvent, PushPayload},
    IndexerJob, IndexerState, IndexingCursor,
};

pub(super) async fn update_evm_empty_cursor(
    client: &HapiCoreEvm,
    contract_address: Address,
) -> Result<IndexerState> {
    tracing::info!("No cursor found searching for the earliest block height");

    let filter = Filter::default()
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .address(contract_address);

    // TODO: make sure it'll work with thousands of transactions; maybe apply paging?
    match client
        .contract
        .client()
        .get_logs(&filter)
        .await?
        .iter()
        .filter_map(|x| x.block_number.as_ref().map(|bn| bn.as_u64()))
        .next()
    {
        Some(block_number) => {
            tracing::info!(block_number, "Earliest block height found");
            Ok(IndexerState::CheckForUpdates {
                cursor: IndexingCursor::Block(block_number),
            })
        }
        None => Ok(IndexerState::Stopped {
            message: "No valid transactions found on the contract address".to_string(),
        }),
    }
}

pub(super) async fn update_evm_block(
    client: &HapiCoreEvm,
    last_block: u64,
    contract_address: Address,
) -> Result<(IndexerState, Vec<IndexerJob>)> {
    let current = client.contract.client().get_block_number().await?.as_u64();
    if last_block < current {
        tracing::info!(from = last_block, to = current, "New blocks found");

        let filter = Filter::default()
            .from_block(BlockNumber::Number(last_block.into()))
            .to_block(BlockNumber::Number(current.into()))
            .address(contract_address);

        // TODO: if fetching fails - stop the indexer - test it
        let mut jobs = vec![];

        for log in client.contract.client().get_logs(&filter).await? {
            let log_header = client.decode_event(&log)?;

            tracing::info!(
                name = log_header.name,
                tx = ?log.transaction_hash,
                block = ?log.block_number,
                tokens = ?log_header.tokens,
                "Found event",
            );
            jobs.push(IndexerJob::Log(log))
        }

        Ok((
            IndexerState::Processing {
                cursor: IndexingCursor::Block(current),
            },
            jobs,
        ))
    } else {
        tracing::debug!("No new events found, waiting for new blocks");
        Ok((
            IndexerState::Waiting {
                until: now()? + client.provider.get_interval().as_secs(),
                cursor: IndexingCursor::Block(last_block),
            },
            Vec::new(),
        ))
    }
}

pub(super) async fn update_evm_transaction(
    client: &HapiCoreEvm,
    hash: String,
) -> Result<IndexerState> {
    match client
        .contract
        .client()
        .get_transaction(hash.parse::<H256>()?)
        .await?
    {
        Some(_) => {
            tracing::info!(tx = hash, "Found transaction");
            Ok(IndexerState::Processing {
                cursor: IndexingCursor::Transaction(hash),
            })
        }
        None => {
            tracing::error!(tx = hash, "Transaction not found");
            Ok(IndexerState::Stopped {
                message: format!("Transaction '{hash}' not found"),
            })
        }
    }
}

pub(super) async fn process_evm_job_log(
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

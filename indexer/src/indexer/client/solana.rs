use {
    anyhow::{bail, Result},
    hapi_core::HapiCoreSolana,
    hapi_core::{
        client::{
            entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
            events::EventName,
            solana::DecodedInstruction,
        },
        get_solana_account,
    },
    solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature},
    std::time::Duration,
    std::{collections::VecDeque, str::FromStr},
    tokio::time::sleep,
};

use crate::indexer::{
    client::indexer_client::{FetchingArtifacts, PAGE_SIZE},
    push::{NetworkData, PushData, PushEvent, PushPayload},
    IndexerJob, IndexingCursor,
};

const REPORTER_ACCOUNT_INDEX: usize = 2;
const CASE_ACCOUNT_INDEX: usize = 3;
const ADDRESS_ACCOUNT_INDEX: usize = 4;
const ASSET_ACCOUNT_INDEX: usize = 4;

async fn get_signature_list(
    client: &HapiCoreSolana,
    signature_cursor: Option<Signature>,
    fetching_delay: Duration,
) -> Result<Vec<IndexerJob>> {
    let mut recent_tx = None;
    let mut signature_list = VecDeque::new();

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: recent_tx,
            until: signature_cursor,
            limit: Some(*PAGE_SIZE as usize),
            commitment: Some(CommitmentConfig::confirmed()),
        };

        tracing::debug!(before = ?config.before, until = ?config.until, "Fetching signatures");

        let signature_batch = client
            .rpc_client
            .get_signatures_for_address_with_config(&client.program_id, config)
            .await?;

        if let Some(recent) = signature_batch.last() {
            recent_tx = Some(Signature::from_str(&recent.signature)?);

            for sign in signature_batch {
                tracing::info!(
                    tx_hash = sign.signature.to_string(),
                    block = sign.block_time,
                    "Found transaction",
                );

                signature_list.push_front(IndexerJob::Transaction(sign.signature.to_string()));
            }

            sleep(fetching_delay).await;
        } else {
            break;
        }
    }

    Ok(signature_list.into())
}

#[tracing::instrument(skip(client, fetching_delay))]
pub(super) async fn fetch_solana_jobs(
    client: &HapiCoreSolana,
    current_cursor: &IndexingCursor,
    fetching_delay: Duration,
) -> Result<FetchingArtifacts> {
    let signature_cursor = match &current_cursor {
        IndexingCursor::None => None,
        IndexingCursor::Transaction(tx) => Some(Signature::from_str(tx)?),
        _ => bail!("Solana network must have a transaction cursor"),
    };

    tracing::info!(
        current_cursor = %current_cursor,
        "Fetching solana jobs"
    );

    let signature_list = get_signature_list(client, signature_cursor, fetching_delay).await?;
    tracing::info!(count = signature_list.len(), "Found jobs");

    let new_cursor = if let Some(recent) = signature_list.last() {
        IndexingCursor::try_from(recent.clone())?
    } else {
        current_cursor.clone()
    };

    Ok(FetchingArtifacts {
        jobs: signature_list,
        cursor: new_cursor,
    })
}

#[tracing::instrument(skip(client, network_data))]
pub(super) async fn process_solana_job(
    client: &HapiCoreSolana,
    signature: &str,
    network_data: NetworkData,
) -> Result<Option<Vec<PushPayload>>> {
    let instructions = client.get_hapi_instructions(signature).await?;

    if instructions.is_empty() {
        tracing::warn!(hash = signature, "Ignoring transaction");

        return Ok(None);
    }

    tracing::info!(signature, "Processing transaction",);

    let mut payloads = vec![];

    for instruction in instructions {
        if let Some(data) = get_instruction_data(client, &instruction).await? {
            tracing::info!(
                name = instruction.name.to_string(),
                signature,
                tx_index = instruction.id,
                block = instruction.blocktime,
                arguments = ?instruction.data,
                "Found instruction",
            );

            payloads.push(PushPayload {
                network_data: network_data.clone(),
                event: PushEvent {
                    name: instruction.name,
                    tx_hash: signature.to_string(),
                    tx_index: instruction.id.into(),
                    timestamp: instruction.blocktime,
                },
                data,
            });
        }
    }

    Ok(Some(payloads))
}

async fn get_instruction_data(
    client: &HapiCoreSolana,
    instruction: &DecodedInstruction,
) -> Result<Option<PushData>> {
    match instruction.name {
        EventName::CreateReporter
        | EventName::UpdateReporter
        | EventName::ActivateReporter
        | EventName::DeactivateReporter
        | EventName::Unstake => {
            let account = get_pubkey(&instruction.account_keys, REPORTER_ACCOUNT_INDEX)?;
            let reporter = get_solana_account!(client, &account, Reporter)?;

            tracing::info!(?reporter.id, "Reporter is created or modified");

            return Ok(Some(reporter.into()));
        }

        EventName::CreateCase | EventName::UpdateCase => {
            let account = get_pubkey(&instruction.account_keys, CASE_ACCOUNT_INDEX)?;
            let case = get_solana_account!(client, &account, Case)?;

            tracing::info!(?case.id, "Case is created or modified");

            return Ok(Some(case.into()));
        }

        EventName::CreateAddress | EventName::UpdateAddress => {
            let account = get_pubkey(&instruction.account_keys, ADDRESS_ACCOUNT_INDEX)?;
            let address = get_solana_account!(client, &account, Address)?;

            tracing::info!(address.address, "Address is created or modified");

            return Ok(Some(address.into()));
        }
        EventName::CreateAsset | EventName::UpdateAsset => {
            let account = get_pubkey(&instruction.account_keys, ASSET_ACCOUNT_INDEX)?;
            let asset = get_solana_account!(client, &account, Asset)?;

            tracing::info!(asset.address, ?asset.asset_id, "Asset is created or modified");

            return Ok(Some(asset.into()));
        }

        EventName::Initialize => {
            tracing::info!("Network created");
        }
        EventName::UpdateStakeConfiguration
        | EventName::UpdateRewardConfiguration
        | EventName::SetAuthority => {
            tracing::info!("Configuration is changed");
        }
        EventName::ConfirmAddress | EventName::ConfirmAsset => {
            tracing::info!("Confirmation is received");
        }
    }

    Ok(None)
}

fn get_pubkey(accounts: &[String], index: usize) -> Result<Pubkey> {
    Ok(Pubkey::from_str(
        accounts
            .get(index)
            .ok_or(anyhow::anyhow!("Account {} is absent", index))?,
    )?)
}

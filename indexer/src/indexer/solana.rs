use {
    anyhow::Result, hapi_core::HapiCoreSolana,
    solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config,
    solana_sdk::commitment_config::CommitmentConfig, std::str::FromStr,
};

use std::collections::VecDeque;

use hapi_core::{
    client::{
        entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
        events::EventName,
        solana::DecodedInstruction,
    },
    get_solana_account,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use super::{
    push::{PushData, PushEvent, PushPayload},
    IndexerJob,
};

const SOLANA_BATCH_SIZE: usize = 500;

// TODO: add valid indexes
const REPORTER_ACCOUNT_INDEX: usize = 1;
const CASE_ACCOUNT_INDEX: usize = 1;
const ADDRESS_ACCOUNT_INDEX: usize = 1;
const ASSET_ACCOUNT_INDEX: usize = 1;

pub(super) async fn update_solana_cursor(
    client: &HapiCoreSolana,
    current_cursor: Option<&str>,
) -> Result<Vec<IndexerJob>> {
    tracing::info!("No cursor found searching for the earliest transaction");
    let mut signature_list = VecDeque::new();
    let mut recent_tx = None;

    let signature_cursor = if let Some(cursor) = current_cursor {
        Some(Signature::from_str(cursor)?)
    } else {
        None
    };

    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: recent_tx,
            until: signature_cursor,
            limit: Some(SOLANA_BATCH_SIZE),
            commitment: Some(CommitmentConfig::confirmed()),
        };

        let signature_batch = client
            .rpc_client
            .get_signatures_for_address_with_config(&client.program_id, config)
            .await?;

        if let Some(recent) = signature_batch.last() {
            recent_tx = Some(Signature::from_str(&recent.signature)?);

            for sign in signature_batch {
                signature_list.push_front(IndexerJob::Transaction(sign.signature.to_string()));
            }
        } else {
            break;
        }
    }

    return Ok(signature_list.into());
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

pub(super) async fn process_solana_job(
    client: &HapiCoreSolana,
    signature: &str,
) -> Result<Option<Vec<PushPayload>>> {
    let instructions = client.get_hapi_instructions(signature).await?;

    if instructions.is_empty() {
        tracing::warn!(hash = signature, "Ignoring transaction");
        return Ok(None);
    }

    tracing::info!(signature, "Processing transaction",);

    let mut payloads = vec![];

    for instruction in instructions {
        if let Some(data) = get_instruction_data(&client, &instruction).await? {
            tracing::info!(
                name = instruction.name.to_string(),
                signature,
                tx_index = instruction.id,
                block = instruction.blocktime,
                arguments = ?instruction.data,
                "Found instruction",
            );

            payloads.push(PushPayload {
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

fn get_pubkey(accounts: &Vec<String>, index: usize) -> Result<Pubkey> {
    Ok(Pubkey::from_str(
        accounts
            .get(index)
            .ok_or(anyhow::anyhow!("Account {} is absent", index))?,
    )?)
}

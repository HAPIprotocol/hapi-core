use std::{io::Write, str::FromStr};
use uuid::Uuid;

use anchor_client::solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};
use solana_cli_config::{Config, CONFIG_FILE};

use crate::client::result::{ClientError, Result};

const BPF_LOADER_UPGRADEABLE_ID: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

pub fn get_signer(private_key: Option<String>) -> Result<Keypair> {
    if let Some(pk) = private_key {
        return Ok(Keypair::from_base58_string(&pk));
    }
    let default_config = CONFIG_FILE
        .as_ref()
        .ok_or(ClientError::AbsentDefaultConfig)?;

    let cli_config =
        Config::load(default_config).map_err(|e| ClientError::UnableToLoadConfig(e.to_string()))?;

    read_keypair_file(cli_config.keypair_path)
        .map_err(|e| ClientError::SolanaKeypairFile(format!("`keypair-path`: {e}")))
}

/// Returns program data account
pub fn get_program_data_address(program_id: &Pubkey) -> Result<Pubkey> {
    Ok(Pubkey::find_program_address(
        &[&program_id.to_bytes()],
        &Pubkey::from_str(BPF_LOADER_UPGRADEABLE_ID)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`bpf-loader`: {e}")))?,
    )
    .0)
}

/// Returns network PDA address
pub fn get_network_address(network_name: &str, program_id: &Pubkey) -> Result<(Pubkey, u8)> {
    let mut name = [0u8; 32];
    byte_array_from_str(network_name, &mut name)?;

    Ok(Pubkey::find_program_address(
        &[b"network", name.as_ref()],
        program_id,
    ))
}

/// Returns reporter PDA address
pub fn get_reporter_address(
    reporter_id: Uuid,
    network: &Pubkey,
    program_id: &Pubkey,
) -> Result<(Pubkey, u8)> {
    let id = reporter_id.as_bytes();

    Ok(Pubkey::find_program_address(
        &[b"reporter", network.as_ref(), id],
        program_id,
    ))
}

/// Returns case PDA address
pub fn get_case_address(
    case_id: Uuid,
    network: &Pubkey,
    program_id: &Pubkey,
) -> Result<(Pubkey, u8)> {
    let id = case_id.as_bytes();

    Ok(Pubkey::find_program_address(
        &[b"case", network.as_ref(), id],
        program_id,
    ))
}

/// Returns address PDA address
pub fn get_address_address(
    address: &[u8; 64],
    network: &Pubkey,
    program_id: &Pubkey,
) -> Result<(Pubkey, u8)> {
    Ok(Pubkey::find_program_address(
        &[
            b"address",
            network.as_ref(),
            address[0..32].as_ref(),
            address[32..64].as_ref(),
        ],
        program_id,
    ))
}

/// Returns asset PDA address
pub fn get_asset_address(
    address: &[u8; 64],
    asset_id: &[u8; 64],
    network: &Pubkey,
    program_id: &Pubkey,
) -> Result<(Pubkey, u8)> {
    Ok(Pubkey::find_program_address(
        &[
            b"asset",
            network.as_ref(),
            address[0..32].as_ref(),
            address[32..64].as_ref(),
            asset_id[0..32].as_ref(),
            asset_id[32..64].as_ref(),
        ],
        program_id,
    ))
}

pub fn byte_array_from_str(data: &str, bytes: &mut [u8]) -> Result<()> {
    let mut bytes = &mut bytes[..];
    bytes
        .write_all(data.as_bytes())
        .map_err(|e| ClientError::InvalidData(e.to_string()))?;

    Ok(())
}

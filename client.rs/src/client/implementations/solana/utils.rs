use anchor_client::solana_sdk::pubkey::Pubkey;
use std::io::Write;

use crate::client::result::{ClientError, Result};

/// Returns network PDA address
pub fn get_network(network_name: &str, program_id: &Pubkey) -> Result<Pubkey> {
    let name = &byte_array_from_str(network_name)?;

    Ok(Pubkey::find_program_address(&[b"network", name.as_ref()], program_id).0)
}

fn byte_array_from_str(data: &str) -> Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    {
        let mut bytes = &mut bytes[..];
        bytes
            .write_all(data.as_bytes())
            .map_err(|e| ClientError::InvalidData(e.to_string()))?;
    }

    Ok(bytes)
}

use super::{utils::Category, DISCRIMINATOR_LENGTH};
use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network account
    pub network: Pubkey,

    /// Asset contract address (i.e. NFT)
    pub address: [u8; 64],

    /// Asset ID on it’s contract
    pub id: [u8; 32],

    /// Primary category of activity detected on the address
    pub category: Category,

    /// Estimated risk score on a scale from 0 to 10 (where 0 is safe and 10 is maximum risk)
    pub risk_score: u8,

    /// Case UUID
    pub case_id: u128,

    /// Reporter UUID
    pub reporter_id: u128,

    /// Confirmation count for this address
    pub confirmations: u8,
}

impl Asset {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 1 + 32 + 64 + 32 + 1 + 1 + 16 + 16 + 1);
    pub const VERSION: u16 = 1;
}

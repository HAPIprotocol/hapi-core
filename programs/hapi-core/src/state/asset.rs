use super::address::Category;
use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Asset {
    /// Account version
    pub version: u16,

    /// Community account, which this address belongs to
    pub community: Pubkey,

    /// Network account, which this address belongs to
    pub network: Pubkey,

    /// Asset mint account
    pub mint: [u8; 64],

    /// Asset ID
    pub asset_id: [u8; 32],

    /// Seed bump for PDA
    pub bump: u8,

    /// ID of the associated case
    pub case_id: u64,

    /// Reporter account public key
    pub reporter: Pubkey,

    /// Category of illicit activity identified with this address
    pub category: Category,

    /// Address risk score 0..10 (0 is safe, 10 is maximum risk)
    pub risk: u8,

    /// Confirmation count for this address
    pub confirmations: u8,

    /// Accumulated payment amount for report
    pub replication_bounty: u64,
}

impl Asset {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + (2 + 32 + 32 + 64 + 32 + 1 + 8 + 32 + 1 + 1 + 1 + 8);
        
    pub const VERSION: u16 = 1;
}

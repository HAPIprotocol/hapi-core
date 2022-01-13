use anchor_lang::prelude::*;

use super::address::Category;

#[account]
pub struct Asset {
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
}

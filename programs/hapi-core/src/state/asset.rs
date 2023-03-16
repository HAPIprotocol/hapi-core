use super::address::Category;
use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

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

    /// Accumulated payment amount for report
    pub replication_bounty: u64,
}

#[account]
pub struct DeprecatedAsset {
    pub community: Pubkey,
    pub network: Pubkey,
    pub mint: [u8; 64],
    pub asset_id: [u8; 32],
    pub bump: u8,
    pub case_id: u64,
    pub reporter: Pubkey,
    pub category: Category,
    pub risk: u8,
    pub confirmations: u8,
}

impl Asset {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (32 + 32 + 64 + 32 + 1 + 8 + 32 + 1 + 1 + 1);

    pub fn from_deprecated(deprecated: DeprecatedAsset) -> Self {
        Self {
            community: deprecated.community,
            network: deprecated.network,
            mint: deprecated.mint,
            asset_id: deprecated.asset_id,
            bump: deprecated.bump,
            case_id: deprecated.case_id,
            reporter: deprecated.reporter,
            category: deprecated.category,
            risk: deprecated.risk,
            confirmations: deprecated.confirmations,
            replication_bounty: 0,
        }
    }
}

use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Community {
    /// Account version
    pub version: u16,

    /// Community authority wallet
    pub authority: Pubkey,

    /// Community ID
    pub community_id: u64,

    /// Seed bump for PDA
    pub bump: u8,

    /// Community case counter
    pub cases: u64,

    /// Number of confirmations needed for address to be considered confirmed
    pub confirmation_threshold: u8,

    /// Number of epochs reporter must wait to retrieve their stake
    pub stake_unlock_epochs: u64,

    /// Stake token mint account
    pub stake_mint: Pubkey,

    /// Amount of stake required from a reporter of validator type
    pub validator_stake: u64,

    /// Amount of stake required from a reporter of tracer type
    pub tracer_stake: u64,

    /// Amount of stake required from a reporter of full type
    pub full_stake: u64,

    /// Amount of stake required from a reporter of authority type
    pub authority_stake: u64,

    /// Amount of stake required from a reporter of appraiser type
    pub appraiser_stake: u64,
}

impl Community {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 8 + 1 + 8 + 1 + 8 + 32 + 8 + 8 + 8 + 8 + 8);
    pub const VERSION: u16 = 1;
}

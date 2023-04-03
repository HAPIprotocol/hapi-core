use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Community {
    /// Account version
    pub version: u16,

    /// Community authority wallet
    pub authority: Pubkey,

    /// Community case counter
    pub cases: u64,

    /// Number of confirmations needed for address to be considered confirmed
    pub confirmation_threshold: u8,

    /// Number of epochs reporter must wait to retrieve their stake
    pub stake_unlock_epochs: u64,

    /// Stake token mint account
    pub stake_mint: Pubkey,

    /// Token signer PDA
    pub token_signer: Pubkey,

    /// Seed bump for token signer PDA
    pub token_signer_bump: u8,

    /// Stake holding token account // owner - community (transfer all funds from old to new and remove token_signer)
    pub token_account: Pubkey,

    // TODO: remove
    /// Token account for reporter fee
    pub treasury_token_account: Pubkey, // The same logic as token_account but with reward_mint from network

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
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + (2 + 32 + 8 + 1 + 8 + 32 + 32 + 1 + 32 + 32 + 8 + 8 + 8 + 8 + 8);

    pub const VERSION: u16 = 1;
}

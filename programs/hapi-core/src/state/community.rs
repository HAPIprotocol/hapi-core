use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Community {
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

    /// Stake holding token account
    pub token_account: Pubkey,

    /// Token account for reporter fee
    pub treasury_token_account: Pubkey,

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

#[account]
pub struct DeprecatedCommunity {
    pub authority: Pubkey,
    pub cases: u64,
    pub confirmation_threshold: u8,
    pub stake_unlock_epochs: u64,
    pub stake_mint: Pubkey,
    pub token_signer: Pubkey,
    pub token_signer_bump: u8,
    pub token_account: Pubkey,
    pub validator_stake: u64,
    pub tracer_stake: u64,
    pub full_stake: u64,
    pub authority_stake: u64,
}

impl Community {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + (32 + 8 + 1 + 8 + 32 + 32 + 1 + 32 + 32 + 8 + 8 + 8 + 8 + 8);

    pub fn from_deprecated(
        deprecated: DeprecatedCommunity,
        treasury_token_account: Pubkey,
        appraiser_stake: u64,
    ) -> Self {
        Self {
            authority: deprecated.authority,
            cases: deprecated.cases,
            confirmation_threshold: deprecated.confirmation_threshold,
            stake_unlock_epochs: deprecated.stake_unlock_epochs,
            stake_mint: deprecated.stake_mint,
            token_signer: deprecated.token_signer,
            token_signer_bump: deprecated.token_signer_bump,
            token_account: deprecated.token_account,
            validator_stake: deprecated.validator_stake,
            tracer_stake: deprecated.tracer_stake,
            full_stake: deprecated.full_stake,
            authority_stake: deprecated.authority_stake,
            treasury_token_account,
            appraiser_stake,
        }
    }
}

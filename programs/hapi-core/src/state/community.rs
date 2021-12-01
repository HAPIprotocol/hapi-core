use anchor_lang::prelude::*;

#[account]
pub struct Community {
    /// Community authority wallet
    pub authority: Pubkey,

    /// Community case counter
    pub cases: u64,

    /// Number of confirmations needed for address to be considered confirmed
    pub confirmation_threshold: u32,

    /// Number of epochs reporter must wait to retrieve their stake
    pub stake_unlock_epochs: u64,

    /// Stake token mint account
    pub stake_mint: Pubkey,

    /// Stake holding token account
    pub token_account: Pubkey,

    /// Amount of stake required from a reporter of validator type
    pub validator_stake: u64,

    /// Amount of stake required from a reporter of tracer type
    pub tracer_stake: u64,

    /// Amount of stake required from a reporter of full type
    pub full_stake: u64,

    /// Amount of stake required from a reporter of authority type
    pub authority_stake: u64,
}

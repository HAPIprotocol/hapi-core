use anchor_lang::prelude::*;

#[account]
pub struct Network {
    /// Community account, which this network belongs to
    pub community: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network name (i.e. ethereum, solana, near)
    pub name: [u8; 32],

    /// Reward token mint account
    pub reward_mint: Pubkey,

    /// Reward signer PDA
    pub reward_signer: Pubkey,

    /// Seed bump for reward signer PDA
    pub reward_signer_bump: u8,

    /// Reward amount for tracers that report to this network
    pub tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm addresses on this network
    pub confirmation_reward: u64,
}

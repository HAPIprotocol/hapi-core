use super::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Network {
    /// Account version
    pub version: u16,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network authority
    pub authority: Pubkey,

    /// Network name (i.e. ethereum, solana, near)
    pub name: [u8; 32],

    /// Network address schema
    pub schema: NetworkSchema,

    /// Stake token mint account
    pub stake_mint: Pubkey,

    /// Stake configuration info
    pub stake_info: StakeConfiguration,

    /// Reward token mint account
    pub reward_mint: Pubkey,

    /// Reward configuration info
    pub reward_info: RewardConfiguration,
}

impl Network {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 1 + 32 + 1 + 32 + 40 + 32 + 32 + 8);
    pub const VERSION: u16 = 1;
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct StakeConfiguration {
    /// Duration in seconds of reporter suspension before the stake can be withdrawn
    pub unlock_duration: u64,

    /// Amount of stake required from a reporter of validator type
    pub validator_stake: u64,

    /// Amount of stake required from a reporter of tracer type
    pub tracer_stake: u64,

    /// Amount of stake required from a reporter of publisher type
    pub publisher_stake: u64,

    /// Amount of stake required from a reporter of authority type
    pub authority_stake: u64,
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub struct RewardConfiguration {
    /// Reward amount for tracers that report addresses to this network
    pub address_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm addresses on this network
    pub address_confirmation_reward: u64,

    /// Reward amount for tracers that report assets to this network
    pub asset_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm assets on this network
    pub asset_confirmation_reward: u64,
}

#[derive(Default, Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum NetworkSchema {
    #[default]
    Plain,
    Solana,
    Ethereum,
    Bitcoin,
    Near,
}

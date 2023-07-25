use crate::utils::DISCRIMINATOR_LENGTH;
use anchor_lang::prelude::*;

#[account]
pub struct Network {
    /// Account version
    pub version: u16,

    /// Community account, which this network belongs to
    pub community: Pubkey,

    /// Seed bump for PDA
    pub bump: u8,

    /// Network name (i.e. ethereum, solana, near)
    pub name: [u8; 32],

    // Network address schema
    pub schema: NetworkSchema,

    /// Reward token mint account
    pub reward_mint: Pubkey,

    /// Reward amount for tracers that report addresses to this network
    pub address_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm addresses on this network
    pub address_confirmation_reward: u64,

    /// Reward amount for tracers that report assets to this network
    pub asset_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm assets on this network
    pub asset_confirmation_reward: u64,

    /// Replication price amount
    pub replication_price: u64,
}

impl Network {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (2 + 32 + 1 + 32 + 1 + 32 + 8 + 8 + 8 + 8 + 8);
    pub const VERSION: u16 = 1;
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

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

    /// Reward amount for tracers that report addresses to this network
    pub address_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm addresses on this network
    pub address_confirmation_reward: u64,

    /// Reward amount for tracers that report assets to this network
    pub asset_tracer_reward: u64,

    /// Reward amount for tracers and validators that confirm assets on this network
    pub asset_confirmation_reward: u64,

    // Network address schema
    pub schema: NetworkSchema,
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum NetworkSchema {
    Solana,
    Ethereum,
    Bitcoin,
}

impl Default for NetworkSchema {
    fn default() -> Self {
        NetworkSchema::Solana
    }
}

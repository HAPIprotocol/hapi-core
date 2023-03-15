use anchor_lang::prelude::*;

#[account]
pub struct Network {
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

    /// Replication price amount
    pub replication_price: u64,
}

#[account]
pub struct DeprecatedNetwork {
    pub community: Pubkey,
    pub bump: u8,
    pub name: [u8; 32],
    pub schema: NetworkSchema,
    pub reward_mint: Pubkey,
    pub reward_signer: Pubkey,
    pub reward_signer_bump: u8,
    pub address_tracer_reward: u64,
    pub address_confirmation_reward: u64,
    pub asset_tracer_reward: u64,
    pub asset_confirmation_reward: u64,
}

impl Network {
    pub fn from_deprecated(deprecated: DeprecatedNetwork) -> Self {
        Self {
            community: deprecated.community,
            bump: deprecated.bump,
            name: deprecated.name,
            schema: deprecated.schema,
            reward_mint: deprecated.reward_mint,
            reward_signer: deprecated.reward_signer,
            reward_signer_bump: deprecated.reward_signer_bump,
            address_tracer_reward: deprecated.address_tracer_reward,
            address_confirmation_reward: deprecated.address_confirmation_reward,
            asset_tracer_reward: deprecated.asset_tracer_reward,
            asset_confirmation_reward: deprecated.asset_confirmation_reward,
            replication_price: 0,
        }
    }
}

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum NetworkSchema {
    Plain,
    Solana,
    Ethereum,
    Bitcoin,
    Near,
}

impl Default for NetworkSchema {
    fn default() -> Self {
        NetworkSchema::Plain
    }
}

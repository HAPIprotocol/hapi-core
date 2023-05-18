use anchor_lang::prelude::*;

mod context;
mod error;
mod state;

use context::*;
use state::network::*;

declare_id!("hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6");

#[program]
pub mod hapi_core_solana {
    use super::*;

    pub fn create_network(
        ctx: Context<CreateNetwork>,
        name: [u8; 32],
        schema: NetworkSchema,
        stake_info: StakeConfiguration,
        reward_info: RewardConfiguration,
        bump: u8,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.bump = bump;
        network.name = name;
        network.authority = ctx.accounts.authority.key();
        network.schema = schema;
        network.reward_mint = ctx.accounts.reward_mint.key();
        network.reward_info = reward_info;
        network.stake_mint = ctx.accounts.stake_mint.key();
        network.stake_info = stake_info;
        network.version = Network::VERSION;

        Ok(())
    }

    pub fn update_configuration(
        ctx: Context<UpdateConfiguration>,
        stake_info: StakeConfiguration,
        reward_info: RewardConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.reward_info = reward_info;
        network.stake_info = stake_info;

        Ok(())
    }

    pub fn set_network_authority(ctx: Context<SetNetworkAuthority>) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.authority = ctx.accounts.new_authority.key();

        Ok(())
    }
}

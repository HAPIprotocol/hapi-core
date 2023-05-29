use anchor_lang::prelude::*;

mod context;
mod error;
mod state;

use context::*;
use state::{network::*, reporter::*};

declare_id!("FgE5ySSi6fbnfYGGRyaeW8y6p8A5KybXPyQ2DdxPCNRk");

#[program]
pub mod hapi_core_solana {
    use super::*;

    pub fn create_network(
        ctx: Context<CreateNetwork>,
        name: [u8; 32],
        stake_info: StakeConfiguration,
        reward_info: RewardConfiguration,
        bump: u8,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.bump = bump;
        network.name = name;
        network.authority = ctx.accounts.authority.key();
        network.reward_mint = ctx.accounts.reward_mint.key();
        network.reward_configuration = reward_info;
        network.stake_mint = ctx.accounts.stake_mint.key();
        network.stake_configuration = stake_info;
        network.version = Network::VERSION;

        Ok(())
    }

    pub fn update_configuration(
        ctx: Context<UpdateConfiguration>,
        stake_configuration: StakeConfiguration,
        reward_configuration: RewardConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.reward_configuration = reward_configuration;
        network.stake_configuration = stake_configuration;

        Ok(())
    }

    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.authority = ctx.accounts.new_authority.key();

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        reporter_id: u64,
        account: Pubkey,
        name: [u8; 32],
        role: ReporterRole,
        url: String,
        bump: u8,
    ) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.bump = bump;
        reporter.id = reporter_id;
        reporter.account = account;
        reporter.name = name;
        reporter.role = role;
        reporter.status = ReporterStatus::Inactive;
        reporter.url = url;
        reporter.stake = 0;
        reporter.version = Reporter::VERSION;

        Ok(())
    }

    pub fn update_reporter(
        ctx: Context<UpdateReporter>,
        account: Pubkey,
        name: [u8; 32],
        role: ReporterRole,
        url: String,
    ) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.account = account;
        reporter.name = name;
        reporter.role = role;
        reporter.url = url;

        Ok(())
    }
}

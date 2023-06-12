use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

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

    pub fn update_stake_configuration(
        ctx: Context<UpdateStakeConfiguration>,
        stake_configuration: StakeConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.stake_configuration = stake_configuration;
        network.stake_mint = ctx.accounts.stake_mint.key();

        Ok(())
    }

    pub fn update_reward_configuration(
        ctx: Context<UpdateRewardConfiguration>,
        reward_configuration: RewardConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.reward_configuration = reward_configuration;
        network.reward_mint = ctx.accounts.reward_mint.key();

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
        name: String,
        role: ReporterRole,
        url: String,
        bump: u8,
    ) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.bump = bump;
        reporter.id = reporter_id;
        reporter.network = ctx.accounts.network.key();
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
        name: String,
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

    pub fn activate_reporter(ctx: Context<ActivateReporter>) -> Result<()> {
        let stake_configuration = &ctx.accounts.network.stake_configuration;
        let reporter = &mut ctx.accounts.reporter;

        let stake = match reporter.role {
            ReporterRole::Validator => stake_configuration.validator_stake,
            ReporterRole::Tracer => stake_configuration.tracer_stake,
            ReporterRole::Publisher => stake_configuration.publisher_stake,
            ReporterRole::Authority => stake_configuration.authority_stake,
            ReporterRole::Appraiser => stake_configuration.appraiser_stake,
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.reporter_stake_token_account.to_account_info(),
                    to: ctx.accounts.network_stake_token_account.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            stake,
        )?;

        reporter.status = ReporterStatus::Active;
        reporter.stake = stake;

        Ok(())
    }

    pub fn deactivate_reporter(ctx: Context<DeactivateReporter>) -> Result<()> {
        let network = &ctx.accounts.network;
        let reporter = &mut ctx.accounts.reporter;

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_timestamp =
            Clock::get()?.unix_timestamp as u64 + network.stake_configuration.unlock_duration;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        let network = &ctx.accounts.network;

        let seeds = &[b"network".as_ref(), network.name.as_ref(), &[network.bump]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.network_stake_token_account.to_account_info(),
                    to: ctx.accounts.reporter_stake_token_account.to_account_info(),
                    authority: network.to_account_info(),
                },
                &[&seeds[..]],
            ),
            reporter.stake,
        )?;

        reporter.status = ReporterStatus::Inactive;
        reporter.unlock_timestamp = 0;
        reporter.stake = 0;

        Ok(())
    }
}

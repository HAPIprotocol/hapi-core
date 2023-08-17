use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

mod context;
mod error;
mod state;

use context::*;
use error::{print_error, ErrorCode};
use state::{confirmation::*, network::*, reporter::*, utils::*};

pub use state::{
    address::Address,
    asset::Asset,
    case::{Case, CaseStatus},
    network::{Network, RewardConfiguration, StakeConfiguration},
    reporter::{ReporterRole, ReporterStatus},
    utils::Category,
};

const UUID_VERSION: usize = 4;

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

        msg!(
            "Network created, data:
            name: {}, stake configuration: {:#?}, reward configuration: {:#?}",
            bytes_to_string(&name)?,
            network.stake_configuration,
            network.reward_configuration
        );

        Ok(())
    }

    pub fn update_stake_configuration(
        ctx: Context<UpdateStakeConfiguration>,
        stake_configuration: StakeConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.stake_configuration = stake_configuration;
        network.stake_mint = ctx.accounts.stake_mint.key();

        msg!(
            "Network stake configuration updated: {:#?}",
            network.stake_configuration,
        );

        Ok(())
    }

    pub fn update_reward_configuration(
        ctx: Context<UpdateRewardConfiguration>,
        reward_configuration: RewardConfiguration,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.reward_configuration = reward_configuration;
        network.reward_mint = ctx.accounts.reward_mint.key();

        msg!(
            "Network reward configuration updated: {:#?}",
            network.reward_configuration,
        );

        Ok(())
    }

    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        let network = &mut ctx.accounts.network;

        msg!(
            "Network authority updated
            from {:#?} to {:#?}",
            network.authority,
            ctx.accounts.new_authority.key()
        );

        network.authority = ctx.accounts.new_authority.key();

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        reporter_id: u128,
        account: Pubkey,
        name: String,
        role: ReporterRole,
        url: String,
        bump: u8,
    ) -> Result<()> {
        let id = uuid::Uuid::from_u128(reporter_id);

        if id.get_version_num() != UUID_VERSION {
            return print_error(ErrorCode::InvalidUUID);
        }

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

        msg!(
            "Reporter created, data:
            id: {}, account: {}, name: {}, role: {:#?}, url: {}",
            id,
            reporter.account,
            reporter.name,
            reporter.role,
            reporter.url,
        );

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

        msg!(
            "Reporter updated, data:
            account: {}, name: {}, role: {:#?}, url: {}",
            reporter.account,
            reporter.name,
            reporter.role,
            reporter.url,
        );

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

        msg!("Reporter stake {} will be charged", stake);

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

        msg!("Reporter activated");

        Ok(())
    }

    pub fn deactivate_reporter(ctx: Context<DeactivateReporter>) -> Result<()> {
        let network = &ctx.accounts.network;
        let reporter = &mut ctx.accounts.reporter;

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_timestamp =
            Clock::get()?.unix_timestamp as u64 + network.stake_configuration.unlock_duration;

        msg!(
            "Reporter deactivated, unlock timestamp: {}",
            reporter.unlock_timestamp
        );

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        let network = &ctx.accounts.network;

        let seeds = &[b"network".as_ref(), network.name.as_ref(), &[network.bump]];

        msg!("Reporter stake {} will be refunded", reporter.stake);

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

        msg!("Reporter is inactive");

        Ok(())
    }

    pub fn create_case(
        ctx: Context<CreateCase>,
        case_id: u128,
        name: String,
        url: String,
        bump: u8,
    ) -> Result<()> {
        let id = uuid::Uuid::from_u128(case_id);

        if id.get_version_num() != UUID_VERSION {
            return print_error(ErrorCode::InvalidUUID);
        }

        let case = &mut ctx.accounts.case;

        case.bump = bump;
        case.id = case_id;
        case.name = name;
        case.network = ctx.accounts.network.key();
        case.reporter_id = ctx.accounts.reporter.id;
        case.status = CaseStatus::Open;
        case.url = url;
        case.version = Case::VERSION;

        msg!(
            "Case created, data:
            id: {}, name: {}, url: {}",
            id,
            case.name,
            case.url,
        );

        Ok(())
    }

    pub fn update_case(
        ctx: Context<UpdateCase>,
        name: String,
        url: String,
        state: CaseStatus,
    ) -> Result<()> {
        let case = &mut ctx.accounts.case;

        case.name = name;
        case.url = url;
        case.status = state;

        msg!(
            "Case updated, data:
            name: {}, url: {}, status: {:#?}",
            case.name,
            case.url,
            case.status,
        );

        Ok(())
    }

    pub fn create_address(
        ctx: Context<CreateAddress>,
        addr: [u8; 64],
        category: Category,
        risk_score: u8,
        bump: u8,
    ) -> Result<()> {
        if risk_score > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let address = &mut ctx.accounts.address;

        address.network = ctx.accounts.network.key();
        address.address = addr;
        address.bump = bump;
        address.category = category;
        address.risk_score = risk_score;
        address.case_id = ctx.accounts.case.id;
        address.reporter_id = ctx.accounts.reporter.id;
        address.version = Address::VERSION;

        msg!(
            "Address created, data:
            address: {}, category: {:#?}, risk score: {}",
            bytes_to_string(&address.address)?,
            address.category,
            address.risk_score,
        );

        Ok(())
    }

    pub fn update_address(
        ctx: Context<UpdateAddress>,
        category: Category,
        risk_score: u8,
    ) -> Result<()> {
        if risk_score > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let address = &mut ctx.accounts.address;

        address.category = category;
        address.risk_score = risk_score;
        address.case_id = ctx.accounts.case.id;

        msg!(
            "Address updated, data:
            category: {:#?}, risk score: {}",
            address.category,
            address.risk_score,
        );

        Ok(())
    }

    pub fn confirm_address(ctx: Context<ConfirmAddress>, bump: u8) -> Result<()> {
        let address = &mut ctx.accounts.address;
        let confirmation = &mut ctx.accounts.confirmation;

        confirmation.network = ctx.accounts.network.key();
        confirmation.bump = bump;
        confirmation.reporter_id = ctx.accounts.reporter.id;
        confirmation.account = address.key();
        confirmation.version = Confirmation::VERSION;

        address.confirmations += 1;

        msg!(
            "Address confirmed by {}, confirmation count: {}",
            ctx.accounts.reporter.id,
            address.confirmations
        );

        Ok(())
    }

    pub fn create_asset(
        ctx: Context<CreateAsset>,
        addr: [u8; 64],
        asset_id: [u8; 64],
        category: Category,
        risk_score: u8,
        bump: u8,
    ) -> Result<()> {
        if risk_score > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let asset = &mut ctx.accounts.asset;

        asset.network = ctx.accounts.network.key();
        asset.address = addr;
        asset.id = asset_id;
        asset.bump = bump;
        asset.category = category;
        asset.risk_score = risk_score;
        asset.case_id = ctx.accounts.case.id;
        asset.reporter_id = ctx.accounts.reporter.id;
        asset.version = Asset::VERSION;

        msg!(
            "Asset created, data:
            address: {}, id: {}, category: {:#?}, risk score: {}",
            bytes_to_string(&asset.address)?,
            bytes_to_string(&asset.id)?,
            asset.category,
            asset.risk_score,
        );

        Ok(())
    }

    pub fn update_asset(
        ctx: Context<UpdateAsset>,
        category: Category,
        risk_score: u8,
    ) -> Result<()> {
        if risk_score > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let asset = &mut ctx.accounts.asset;

        asset.category = category;
        asset.risk_score = risk_score;
        asset.case_id = ctx.accounts.case.id;

        msg!(
            "Asset updated, data:
            category: {:#?}, risk score: {}",
            asset.category,
            asset.risk_score,
        );

        Ok(())
    }

    pub fn confirm_asset(ctx: Context<ConfirmAsset>, bump: u8) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        let confirmation = &mut ctx.accounts.confirmation;

        confirmation.network = ctx.accounts.network.key();
        confirmation.bump = bump;
        confirmation.reporter_id = ctx.accounts.reporter.id;
        confirmation.account = asset.key();
        confirmation.version = Confirmation::VERSION;

        asset.confirmations += 1;

        msg!(
            "Asset confirmed by {}, confirmation count: {}",
            ctx.accounts.reporter.id,
            asset.confirmations
        );

        Ok(())
    }
}

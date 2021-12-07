use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod context;
mod error;
mod state;

use context::*;
use error::{print_error, ErrorCode};
use state::{
    address::Category,
    case::CaseStatus,
    reporter::{ReporterRole, ReporterStatus},
};

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize_community(
        ctx: Context<InitializeCommunity>,
        stake_unlock_epochs: u64,
        confirmation_threshold: u32,
        validator_stake: u64,
        tracer_stake: u64,
        full_stake: u64,
        authority_stake: u64,
        signer_bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: InitializeCommunity");

        let community = &mut ctx.accounts.community;

        msg!(
            "token account owner: {:?}",
            ctx.accounts.token_account.owner
        );

        community.authority = *ctx.accounts.authority.key;
        community.cases = 0;
        community.stake_unlock_epochs = stake_unlock_epochs;
        community.confirmation_threshold = confirmation_threshold;
        community.stake_mint = ctx.accounts.stake_mint.to_account_info().key();
        community.token_signer = ctx.accounts.token_signer.key();
        community.token_signer_bump = signer_bump;
        community.token_account = ctx.accounts.token_account.key();
        community.validator_stake = validator_stake;
        community.tracer_stake = tracer_stake;
        community.full_stake = full_stake;
        community.authority_stake = authority_stake;

        Ok(())
    }

    pub fn update_community(
        ctx: Context<UpdateCommunity>,
        stake_unlock_epochs: u64,
        confirmation_threshold: u32,
        validator_stake: u64,
        tracer_stake: u64,
        full_stake: u64,
        authority_stake: u64,
    ) -> ProgramResult {
        msg!("Instruction: UpdateCommunity");

        let community = &mut ctx.accounts.community;

        community.stake_unlock_epochs = stake_unlock_epochs;
        community.confirmation_threshold = confirmation_threshold;
        community.validator_stake = validator_stake;
        community.tracer_stake = tracer_stake;
        community.full_stake = full_stake;
        community.authority_stake = authority_stake;

        Ok(())
    }

    pub fn set_community_authority(ctx: Context<SetCommunityAuthority>) -> ProgramResult {
        msg!("Instruction: SetCommunityAuthority");

        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.new_authority.key;

        Ok(())
    }

    pub fn create_network(
        ctx: Context<CreateNetwork>,
        name: [u8; 32],
        tracer_reward: u64,
        confirmation_reward: u64,
        bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: CreateNetwork");

        let network = &mut ctx.accounts.network;

        network.community = ctx.accounts.community.key();
        network.bump = bump;

        network.name = name;
        network.tracer_reward = tracer_reward;
        network.confirmation_reward = confirmation_reward;

        Ok(())
    }

    pub fn update_network(
        ctx: Context<UpdateNetwork>,
        tracer_reward: u64,
        confirmation_reward: u64,
    ) -> ProgramResult {
        msg!("Instruction: UpdateNetwork");

        let network = &mut ctx.accounts.network;

        network.tracer_reward = tracer_reward;
        network.confirmation_reward = confirmation_reward;

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        role: ReporterRole,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: CreateReporter");

        let reporter = &mut ctx.accounts.reporter;

        reporter.community = ctx.accounts.community.key();
        reporter.pubkey = *ctx.accounts.pubkey.key;
        reporter.bump = bump;

        reporter.role = role;
        reporter.status = ReporterStatus::Inactive;
        reporter.name = name;
        reporter.is_frozen = false;
        reporter.stake = 0;

        Ok(())
    }

    pub fn update_reporter(
        ctx: Context<UpdateReporter>,
        role: ReporterRole,
        name: [u8; 32],
    ) -> ProgramResult {
        msg!("Instruction: UpdateReporter");

        let reporter = &mut ctx.accounts.reporter;

        reporter.role = role;
        reporter.name = name;

        Ok(())
    }

    pub fn create_case(
        ctx: Context<CreateCase>,
        case_id: u64,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: CreateCase");

        let community = &mut ctx.accounts.community;

        if case_id != community.cases + 1 {
            return print_error(ErrorCode::NonSequentialCaseId);
        } else {
            community.cases = case_id;
        }

        let case = &mut ctx.accounts.case;

        case.community = ctx.accounts.community.key();
        case.id = case_id;
        case.bump = bump;

        case.name = name;
        case.status = CaseStatus::Open;
        case.reporter = ctx.accounts.reporter.key();

        Ok(())
    }

    pub fn update_case(
        ctx: Context<UpdateCase>,
        name: [u8; 32],
        status: CaseStatus,
    ) -> ProgramResult {
        msg!("Instruction: UpdateCase");

        let case = &mut ctx.accounts.case;

        case.name = name;
        case.status = status;

        Ok(())
    }

    pub fn create_address(
        ctx: Context<CreateAddress>,
        pubkey: Pubkey,
        category: Category,
        risk: u8,
        bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: CreateAddress");

        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let address = &mut ctx.accounts.address;

        address.network = ctx.accounts.network.key();
        address.address = pubkey;
        address.bump = bump;

        address.community = ctx.accounts.community.key();
        address.reporter = ctx.accounts.reporter.key();
        address.case_id = ctx.accounts.case.id;
        address.category = category;
        address.risk = risk;
        address.confirmations = 0;

        Ok(())
    }

    pub fn confirm_address(ctx: Context<ConfirmAddress>) -> ProgramResult {
        msg!("Instruction: ConfirmAddress");

        let address = &mut ctx.accounts.address;

        address.confirmations += 1;

        Ok(())
    }

    pub fn update_address(
        ctx: Context<UpdateAddress>,
        category: Category,
        risk: u8,
    ) -> ProgramResult {
        msg!("Instruction: UpdateAddress");

        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let address = &mut ctx.accounts.address;

        address.risk = risk;
        address.category = category;

        Ok(())
    }

    pub fn create_asset(
        ctx: Context<CreateAsset>,
        mint: Pubkey,
        asset_id: [u8; 32],
        category: Category,
        risk: u8,
        bump: u8,
    ) -> ProgramResult {
        msg!("Instruction: CreateAsset");

        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let asset = &mut ctx.accounts.asset;

        asset.network = ctx.accounts.network.key();
        asset.mint = mint;
        asset.asset_id = asset_id;
        asset.bump = bump;

        asset.community = ctx.accounts.community.key();
        asset.reporter = ctx.accounts.reporter.key();
        asset.case_id = ctx.accounts.case.id;
        asset.category = category;
        asset.risk = risk;
        asset.confirmations = 0;

        Ok(())
    }

    pub fn update_asset(ctx: Context<UpdateAsset>, category: Category, risk: u8) -> ProgramResult {
        msg!("Instruction: UpdateAsset");

        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        let asset = &mut ctx.accounts.asset;

        asset.risk = risk;
        asset.category = category;

        Ok(())
    }

    pub fn activate_reporter(ctx: Context<ActivateReporter>) -> ProgramResult {
        msg!("Instruction: ActivateReporter");

        let community = &ctx.accounts.community;

        let reporter = &mut ctx.accounts.reporter;

        let stake = match reporter.role {
            ReporterRole::Validator => community.validator_stake,
            ReporterRole::Tracer => community.tracer_stake,
            ReporterRole::Full => community.full_stake,
            ReporterRole::Authority => community.authority_stake,
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reporter_token_account.to_account_info(),
                to: ctx.accounts.community_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );

        token::transfer(cpi_context, stake)?;

        reporter.status = ReporterStatus::Active;
        reporter.stake = stake;

        Ok(())
    }

    pub fn deactivate_reporter(ctx: Context<DeactivateReporter>) -> ProgramResult {
        msg!("Instruction: DeactivateReporter");

        let community = &ctx.accounts.community;

        let reporter = &mut ctx.accounts.reporter;

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_epoch = Clock::get()?.epoch + community.stake_unlock_epochs;

        Ok(())
    }

    pub fn release_reporter(ctx: Context<ReleaseReporter>) -> ProgramResult {
        msg!("Instruction: ReleaseReporter");

        let reporter = &mut ctx.accounts.reporter;

        if reporter.unlock_epoch > Clock::get()?.epoch {
            return print_error(ErrorCode::ReleaseEpochInFuture);
        }

        let community = ctx.accounts.community.clone();

        let token_signer = ctx.accounts.community_token_signer.clone();

        let seeds = &[
            b"community_stash".as_ref(),
            community.to_account_info().key.as_ref(),
            &[community.token_signer_bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.community_token_account.to_account_info(),
                to: ctx.accounts.reporter_token_account.to_account_info(),
                authority: token_signer.to_account_info(),
            },
            signer,
        );

        token::transfer(cpi_context, reporter.stake)?;

        reporter.status = ReporterStatus::Inactive;
        reporter.unlock_epoch = 0;
        reporter.stake = 0;

        Ok(())
    }

    pub fn freeze_reporter(ctx: Context<FreezeReporter>) -> ProgramResult {
        msg!("Instruction: FreezeReporter");

        let reporter = &mut ctx.accounts.reporter;

        reporter.is_frozen = true;

        Ok(())
    }

    pub fn unfreeze_reporter(ctx: Context<UnfreezeReporter>) -> ProgramResult {
        msg!("Instruction: UnfreezeReporter");

        let reporter = &mut ctx.accounts.reporter;

        reporter.is_frozen = false;

        Ok(())
    }
}

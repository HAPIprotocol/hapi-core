use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod context;
mod error;
mod state;

use context::*;
use error::ErrorCode;
use state::{
    address::Category,
    case::CaseStatus,
    reporter::{ReporterStatus, ReporterType},
};

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.authority.key;
        community.cases = 0;

        Ok(())
    }

    pub fn create_network(ctx: Context<CreateNetwork>, name: [u8; 32], bump: u8) -> ProgramResult {
        let network = &mut ctx.accounts.network;

        network.community = ctx.accounts.community.key();
        network.bump = bump;

        network.name = name;

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        reporter_type: ReporterType,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        let reporter = &mut ctx.accounts.reporter;

        reporter.community = ctx.accounts.community.key();
        reporter.bump = bump;

        reporter.pubkey = *ctx.accounts.pubkey.key;
        reporter.reporter_type = reporter_type;
        reporter.reporter_status = ReporterStatus::OnHold;
        reporter.name = name;

        Ok(())
    }

    pub fn create_case(
        ctx: Context<CreateCase>,
        case_id: u64,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        let community = &mut ctx.accounts.community;

        if case_id != community.cases + 1 {
            return Err(ErrorCode::NonSequentialCaseId.into());
        } else {
            community.cases = case_id;
        }

        let case = &mut ctx.accounts.case;

        case.community = ctx.accounts.community.key();
        case.bump = bump;

        case.name = name;
        case.status = CaseStatus::Open;
        case.id = case_id;
        case.reporter = ctx.accounts.reporter.key();

        Ok(())
    }

    pub fn create_address(
        ctx: Context<CreateAddress>,
        pubkey: Pubkey,
        category: Category,
        risk: u8,
        bump: u8,
    ) -> ProgramResult {
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
}

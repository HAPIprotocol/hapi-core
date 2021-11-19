use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod context;
mod error;
mod state;

use context::*;
use state::reporter::ReporterType;

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.authority.key;
        community.case_count = 0;

        Ok(())
    }

    pub fn create_network(ctx: Context<CreateNetwork>, name: [u8; 32], bump: u8) -> ProgramResult {
        let network = &mut ctx.accounts.network;

        network.name = name;
        network.bump = bump;

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        reporter_type: ReporterType,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        let reporter = &mut ctx.accounts.reporter;

        reporter.pubkey = *ctx.accounts.pubkey.key;
        reporter.reporter_type = reporter_type;
        reporter.name = name;
        reporter.bump = bump;

        Ok(())
    }
}

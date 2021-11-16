use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let community = &mut ctx.accounts.community;
        community.authority = *ctx.accounts.authority.key;
        community.case_count = 0;
        Ok(())
    }

    pub fn create_network(
        ctx: Context<CreateNetwork>,
        name: [u8; 32],
        bump: u8,
    ) -> ProgramResult {
        let community = &ctx.accounts.community;

        if community.authority.key() != *ctx.accounts.authority.key {
            return Err(ErrorCode::Unauthorized.into());
        }

        let network = &mut ctx.accounts.network;

        network.name = name;
        network.bump = bump;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, owner = id(), space = 8 + 32 + 8)]
    pub community: Account<'info, Community>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], bump: u8)]
pub struct CreateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(init, payer = authority, owner = id(), seeds = [b"network", community.key().as_ref(), &name], bump = bump, space = 8 + 1 + 32)]
    pub network: Account<'info, Network>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Community {
    pub authority: Pubkey,
    pub case_count: u64,
}

#[account]
pub struct Network {
    pub name: [u8; 32],
    pub bump: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Unexpected account has been used.")]
    UnexpectedAccount,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}

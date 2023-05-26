use {
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};

use crate::{
    error::ErrorCode,
    id,
    program::HapiCoreSolana,
    state::{network::*, reporter::*, ACCOUNT_RESERVE_SPACE},
};

#[derive(Accounts)]
#[instruction(
    name: [u8; 32],
    bump: u8,
)]
pub struct CreateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"network".as_ref(), &name],
        bump,
        space = Network::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub network: Account<'info, Network>,

    #[account(
        mut,
        owner = Token::id(),
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        mut,
        owner = Token::id(),
    )]
    pub stake_mint: Account<'info, Mint>,

    #[account(
        constraint = stake_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = stake_token_account.owner == network.key() @ ErrorCode::IllegalOwner,
        owner = Token::id(),
    )]
    pub stake_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = program_account.key() == id() @ ErrorCode::InvalidProgramAccount,
        constraint = program_account.programdata_address()? == Some(program_data.key()) @ ErrorCode::InvalidProgramData,
    )]
    pub program_account: Program<'info, HapiCoreSolana>,

    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key()) @ ErrorCode::AuthorityMismatch,
    )]
    pub program_data: Account<'info, ProgramData>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfiguration<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(
        constraint = network.authority == authority.key() ||  program_data.upgrade_authority_address == Some(authority.key()) @ ErrorCode::AuthorityMismatch,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    /// CHECK: this account is not dangerous
    #[account(
        constraint = new_authority.key() != authority.key() @ ErrorCode::AuthorityMismatch,
    )]
    pub new_authority: AccountInfo<'info>,

    #[account(
        constraint = program_account.key() == id() @ ErrorCode::InvalidProgramAccount,
        constraint = program_account.programdata_address()? == Some(program_data.key()) @ ErrorCode::InvalidProgramData,
    )]
    pub program_account: Program<'info, HapiCoreSolana>,

    pub program_data: Account<'info, ProgramData>,
}

#[derive(Accounts)]
#[instruction(
    reporter_id: u64,
    bump: u8,
)]
pub struct CreateReporter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter_id.to_le_bytes()],
        bump,
        space = Reporter::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub reporter: Account<'info, Reporter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateReporter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        mut,
        owner = id(),
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_le_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

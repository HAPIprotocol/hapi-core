use {
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};

use crate::{
    error::ErrorCode,
    id,
    state::{network::*, ACCOUNT_RESERVE_SPACE},
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

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

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
pub struct SetNetworkAuthority<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    /// CHECK: this account is not dangerous
    #[account(
        constraint = new_authority.key() != authority.key() @ ErrorCode::AuthorityMismatch,
    )]
    pub new_authority: AccountInfo<'info>,
}

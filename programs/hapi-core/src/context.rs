use anchor_lang::prelude::*;

use crate::{
    id,
    state::{
        community::Community,
        network::Network,
        reporter::{Reporter, ReporterType},
    },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, owner = id(), space = 100)]
    pub community: Account<'info, Community>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], bump: u8)]
pub struct CreateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(owner = id(), has_one = authority)]
    pub community: Account<'info, Community>,

    #[account(init, payer = authority, owner = id(), seeds = [b"network", community.key().as_ref(), &name], bump = bump, space = 100)]
    pub network: Account<'info, Network>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], reporter_type: ReporterType, bump: u8)]
pub struct CreateReporter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(owner = id(), has_one = authority)]
    pub community: Account<'info, Community>,

    #[account(init, payer = authority, owner = id(), seeds = [b"reporter", community.key().as_ref(), pubkey.key().as_ref()], bump = bump, space = 100 )]
    pub reporter: Account<'info, Reporter>,

    pub pubkey: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

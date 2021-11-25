use anchor_lang::prelude::*;

use crate::{
    id,
    state::{
        case::Case,
        community::Community,
        network::Network,
        reporter::{Reporter, ReporterType},
    },
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        space = 100
    )]
    pub community: Account<'info, Community>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], bump: u8)]
pub struct CreateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority
    )]
    pub community: Account<'info, Community>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"network", community.key().as_ref(), &name],
        bump = bump,
        space = 100
    )]
    pub network: Account<'info, Network>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], reporter_type: ReporterType, bump: u8)]
pub struct CreateReporter<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority
    )]
    pub community: Account<'info, Community>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"reporter", community.key().as_ref(), pubkey.key().as_ref()],
        bump = bump,
        space = 200
    )]
    pub reporter: Account<'info, Reporter>,

    pub pubkey: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(case_id: u64, name: [u8; 32], bump: u8)]
pub struct CreateCase<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        owner = id()
    )]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community,
        constraint = (reporter.reporter_type == ReporterType::Full || reporter.reporter_type == ReporterType::Authority) && reporter.pubkey == sender.key()
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"case", community.key().as_ref(), &case_id.to_le_bytes()],
        bump = bump,
        space = 200
    )]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}

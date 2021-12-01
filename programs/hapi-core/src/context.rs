use anchor_lang::prelude::*;

use crate::{
    id,
    state::{
        address::{Address, Category},
        asset::Asset,
        case::{Case, CaseStatus},
        community::Community,
        network::Network,
        reporter::{Reporter, ReporterRole, ReporterStatus},
    },
};

#[derive(Accounts)]
#[instruction(stake_unlock_epochs: u32, confirmation_threshold: u32)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        space = 256
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
        seeds = [b"network".as_ref(), community.key().as_ref(), &name],
        bump = bump,
        space = 200
    )]
    pub network: Account<'info, Network>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], role: ReporterRole, bump: u8)]
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
        seeds = [b"reporter".as_ref(), community.key().as_ref(), pubkey.key().as_ref()],
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
        constraint = (reporter.role == ReporterRole::Full
            || reporter.role == ReporterRole::Authority)
            && reporter.pubkey == sender.key()
            && reporter.status == ReporterStatus::Active
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"case".as_ref(), community.key().as_ref(), &case_id.to_le_bytes()],
        bump = bump,
        space = 200
    )]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pubkey: Pubkey, category: Category, risk: u8, bump: u8)]
pub struct CreateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community,
        constraint = (reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Full
            || reporter.role == ReporterRole::Authority)
            && reporter.pubkey == sender.key()
            && reporter.status == ReporterStatus::Active
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community,
        constraint = case.status == CaseStatus::Open
    )]
    pub case: Account<'info, Case>,

    #[account(
        init,
        owner = id(),
        payer = sender,
        seeds = [b"address".as_ref(), network.key().as_ref(), pubkey.as_ref()],
        bump = bump,
        space = 148
    )]
    pub address: Account<'info, Address>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(mint: Pubkey, asset_id: [u8; 32], category: Category, risk: u8, bump: u8)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community,
        constraint = (reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Full
            || reporter.role == ReporterRole::Authority)
            && reporter.pubkey == sender.key()
            && reporter.status == ReporterStatus::Active
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community,
        constraint = case.status == CaseStatus::Open
    )]
    pub case: Account<'info, Case>,

    #[account(
        init,
        owner = id(),
        payer = sender,
        seeds = [b"asset".as_ref(), network.key().as_ref(), mint.as_ref(), asset_id.as_ref()],
        bump = bump,
        space = 180
    )]
    pub asset: Account<'info, Asset>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateReporter<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community,
        constraint = reporter.status == ReporterStatus::Inactive
            && reporter.pubkey == sender.key(),
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct DeactivateReporter<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community,
        constraint = reporter.status == ReporterStatus::Active
            && reporter.pubkey == sender.key(),
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct ReleaseReporter<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community,
        constraint = reporter.status == ReporterStatus::Unstaking
            && reporter.pubkey == sender.key(),
    )]
    pub reporter: Account<'info, Reporter>,
}

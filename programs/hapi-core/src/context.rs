use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    error::ErrorCode,
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
#[instruction(
    stake_unlock_epochs: u64,
    confirmation_threshold: u32,
    validator_stake: u64,
    tracer_stake: u64,
    full_stake: u64,
    authority_stake: u64,
    stash_bump: u8,
)]
pub struct InitializeCommunity<'info> {
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        space = 256
    )]
    pub community: Account<'info, Community>,

    #[account(owner = Token::id())]
    pub stake_mint: Account<'info, Mint>,

    pub token_signer: AccountInfo<'info>,

    #[account(
        mut,
        constraint = token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = token_account.owner == token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    stake_unlock_epochs: u64,
    confirmation_threshold: u32,
    validator_stake: u64,
    tracer_stake: u64,
    full_stake: u64,
    authority_stake: u64,
)]
pub struct UpdateCommunity<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,
}

#[derive(Accounts)]
pub struct SetCommunityAuthority<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        constraint = new_authority.key() != authority.key() @ ErrorCode::AuthorityMismatch,
    )]
    pub new_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(name: [u8; 32], tracer_reward: u64, confirmation_reward: u64, bump: u8)]
pub struct CreateNetwork<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
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
#[instruction(tracer_reward: u64, confirmation_reward: u64)]
pub struct UpdateNetwork<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,
}

#[derive(Accounts)]
#[instruction(role: ReporterRole, name: [u8; 32], bump: u8)]
pub struct CreateReporter<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
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
#[instruction(role: ReporterRole, name: [u8; 32])]
pub struct UpdateReporter<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct FreezeReporter<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct UnfreezeReporter<'info> {
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
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
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Full || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
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
#[instruction(name: [u8; 32], status: CaseStatus)]
pub struct UpdateCase<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        owner = id()
    )]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = (reporter.role == ReporterRole::Full
            && case.reporter == reporter.key())
            || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        has_one = community,
        owner = id(),
        seeds = [b"case".as_ref(), community.key().as_ref(), &case.id.to_le_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,
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
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Full
            || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), community.key().as_ref(), &case.id.to_le_bytes()],
        bump = case.bump,
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
#[instruction(category: Category, risk: u8)]
pub struct UpdateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Full
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), community.key().as_ref(), &case.id.to_le_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        constraint = case.id == address.case_id @ ErrorCode::CaseMismatch,
        has_one = network @ ErrorCode::NetworkMismatch,
        seeds = [b"address".as_ref(), network.key().as_ref(), address.address.as_ref()],
        bump = address.bump,
    )]
    pub address: Account<'info, Address>,
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
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Full
            || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), community.key().as_ref(), &case.id.to_le_bytes()],
        bump = case.bump,
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
#[instruction(category: Category, risk: u8)]
pub struct UpdateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Full
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), community.key().as_ref(), &case.id.to_le_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        constraint = case.id == asset.case_id @ ErrorCode::CaseMismatch,
        has_one = network @ ErrorCode::NetworkMismatch,
        seeds = [b"asset".as_ref(), network.key().as_ref(), asset.mint.as_ref(), asset.asset_id.as_ref()],
        bump = asset.bump,
    )]
    pub asset: Account<'info, Asset>,
}

#[derive(Accounts)]
pub struct ActivateReporter<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(owner = id())]
    pub community: Account<'info, Community>,

    #[account(
        constraint = community.stake_mint == stake_mint.key() @ ErrorCode::InvalidMint
    )]
    pub stake_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = reporter_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = community_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
    )]
    pub community_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.status == ReporterStatus::Inactive @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
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
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
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
        constraint = community.stake_mint == stake_mint.key() @ ErrorCode::InvalidMint
    )]
    pub stake_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = reporter_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"community_stash".as_ref(), community.key().as_ref()],
        bump = community.token_signer_bump,
    )]
    pub community_token_signer: AccountInfo<'info>,

    #[account(
        mut,
        constraint = community_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = community_token_account.owner == community_token_signer.key() @ ProgramError::IllegalOwner,
    )]
    pub community_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.status == ReporterStatus::Unstaking @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

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
        network::{Network, NetworkSchema},
        reporter::{Reporter, ReporterReward, ReporterRole, ReporterStatus},
    },
};

#[derive(Accounts)]
#[instruction(
    stake_unlock_epochs: u64,
    confirmation_threshold: u8,
    validator_stake: u64,
    tracer_stake: u64,
    full_stake: u64,
    authority_stake: u64,
    stash_bump: u8,
)]
pub struct InitializeCommunity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        constraint = token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = token_account.owner == token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = treasury_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = treasury_token_account.owner == token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        space = Community::LEN + 32
    )]
    pub community: Account<'info, Community>,

    #[account(owner = Token::id())]
    pub stake_mint: Account<'info, Mint>,

    /// CHECK: this account is not dangerous
    pub token_signer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    stake_unlock_epochs: u64,
    confirmation_threshold: u8,
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
pub struct MigrateCommunity<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: this account is not dangerous
    #[account(
        mut,
        owner = id()
    )]
    pub community: AccountInfo<'info>,

    // TODO: Remove treasury token account
    #[account(
            constraint = treasury_token_account.mint == stake_mint.key() @ ErrorCode::InvalidToken,
            constraint = treasury_token_account.owner == token_signer.key() @ ProgramError::IllegalOwner,
            owner = Token::id(),
        )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(owner = Token::id())]
    pub stake_mint: Account<'info, Mint>,

    /// CHECK: this account is not dangerous
    pub token_signer: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
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

    /// CHECK: this account is not dangerous
    #[account(
        constraint = new_authority.key() != authority.key() @ ErrorCode::AuthorityMismatch,
    )]
    pub new_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(
    name: [u8; 32],
    schema: NetworkSchema,
    address_tracer_reward: u64,
    address_confirmation_reward: u64,
    asset_tracer_reward: u64,
    asset_confirmation_reward: u64,
    bump: u8,
    reward_signer_bump: u8,
)]
pub struct CreateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    #[account(
        mut,
        owner = Token::id(),
    )]
    pub reward_mint: Account<'info, Mint>,

    /// CHECK: this account is not dangerous
    #[account(
        seeds = [b"network_reward".as_ref(), network.key().as_ref()],
        bump = reward_signer_bump,
    )]
    pub reward_signer: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"network".as_ref(), community.key().as_ref(), &name],
        bump,
        space = Network::LEN + 32
    )]
    pub network: Account<'info, Network>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    address_tracer_reward: u64,
    address_confirmation_reward: u64,
    asset_tracer_reward: u64,
    asset_confirmation_reward: u64,
)]
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
pub struct MigrateNetwork<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        owner = id(),
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Account<'info, Community>,

    /// CHECK: this account is not dangerous
    #[account(
            mut,
            owner = id()
        )]
    pub network: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(role: ReporterRole, name: [u8; 32], bump: u8)]
pub struct CreateReporter<'info> {
    #[account(mut)]
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
        bump,
        space = Reporter::LEN + 32
    )]
    pub reporter: Account<'info, Reporter>,

    /// CHECK: this account is not dangerous
    pub pubkey: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeReporterReward<'info> {
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
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump,
        space = ReporterReward::LEN + 32
    )]
    pub reporter_reward: Account<'info, ReporterReward>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrateReporterReward<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

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
        constraint = reporter.pubkey == authority.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        init,
        payer = authority,
        owner = id(),
        seeds = [b"reporter_reward2".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump,
        space = ReporterReward::LEN + 32
    )]
    pub reporter_reward: Account<'info, ReporterReward>,

    /// CHECK: this account is not dangerous
    #[account(
        mut,
        owner = id(),
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump,
    )]
    pub deprecated_reporter_reward: AccountInfo<'info>,

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
pub struct MigrateReporter<'info> {
    #[account(mut)]
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
        realloc = Reporter::LEN + 32,
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub reporter: Account<'info, Reporter>,

    pub system_program: Program<'info, System>,
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
        constraint = reporter.role == ReporterRole::Publisher || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
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
        bump,
        space = Case::LEN + 32
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
        constraint = (reporter.role == ReporterRole::Publisher
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
pub struct MigrateCase<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        owner = id()
    )]
    pub community: Account<'info, Community>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key())
            || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == authority.key() @ ErrorCode::InvalidReporter,
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
        realloc = Case::LEN + 32,
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(addr: [u8; 64], category: Category, risk: u8, bump: u8)]
pub struct CreateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Publisher
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
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            addr[0..32].as_ref(),
            addr[32..64].as_ref(),
        ],
        bump,
        space = Address::LEN + 32
    )]
    pub address: Account<'info, Address>,

    #[account(
        mut,
        constraint = reporter_payment_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_payment_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_payment_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = treasury_token_account.owner == community.token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(category: Category, risk: u8)]
pub struct UpdateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
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
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            address.address[0..32].as_ref(),
            address.address[32..64].as_ref(),
        ],
        bump = address.bump,
    )]
    pub address: Account<'info, Address>,

    #[account(
        mut,
        constraint = reporter_payment_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_payment_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_payment_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = treasury_token_account.owner == community.token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MigrateAddress<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == authority.key() @ ErrorCode::InvalidReporter,
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

    /// CHECK: this account is not dangerous
    #[account(
        mut,
        owner = id()
    )]
    pub address: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeAddressCase<'info> {
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
            || (reporter.role == ReporterRole::Publisher
            && new_case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
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
        constraint = new_case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        constraint = new_case.id != address.case_id @ ErrorCode::SameCase,
        seeds = [b"case".as_ref(), community.key().as_ref(), &new_case.id.to_le_bytes()],
        bump = new_case.bump,
    )]
    pub new_case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        has_one = network @ ErrorCode::NetworkMismatch,
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            address.address[0..32].as_ref(),
            address.address[32..64].as_ref(),
        ],
        bump = address.bump,
    )]
    pub address: Account<'info, Address>,
}

#[derive(Accounts)]
pub struct ConfirmAddress<'info> {
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
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        owner = id(),
        has_one = reporter,
        has_one = network,
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump = reporter_reward.bump,
    )]
    pub reporter_reward: Account<'info, ReporterReward>,

    #[account(
        mut,
        owner = id(),
        has_one = network,
        constraint = address_reporter_reward.reporter == address.reporter @ ErrorCode::InvalidReporter,
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), address.reporter.as_ref()],
        bump = address_reporter_reward.bump,
    )]
    pub address_reporter_reward: Account<'info, ReporterReward>,

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
        constraint = address.reporter != reporter.key() @ ErrorCode::Unauthorized,
        has_one = network @ ErrorCode::NetworkMismatch,
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            address.address[0..32].as_ref(),
            address.address[32..64].as_ref(),
        ],
        bump = address.bump,
    )]
    pub address: Account<'info, Address>,
}

#[derive(Accounts)]
#[instruction(mint: [u8; 64], asset_id: [u8; 32], category: Category, risk: u8, bump: u8)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Tracer
            || reporter.role == ReporterRole::Publisher
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
        payer = sender,
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            mint[0..32].as_ref(),
            mint[32..64].as_ref(),
            asset_id.as_ref(),
        ],
        bump,
        space = Asset::LEN + 32
    )]
    pub asset: Box<Account<'info, Asset>>,

    #[account(
        mut,
        constraint = reporter_payment_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_payment_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_payment_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = treasury_token_account.owner == community.token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(category: Category, risk: u8)]
pub struct UpdateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
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
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            asset.mint[0..32].as_ref(),
            asset.mint[32..64].as_ref(),
            asset.asset_id.as_ref(),
        ],
        bump = asset.bump,
    )]
    pub asset: Account<'info, Asset>,

    #[account(
        mut,
        constraint = reporter_payment_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_payment_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_payment_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = treasury_token_account.mint == community.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = treasury_token_account.owner == community.token_signer.key() @ ProgramError::IllegalOwner,
        owner = Token::id(),
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MigrateAsset<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority @ ErrorCode::AuthorityMismatch,
    )]
    pub community: Box<Account<'info, Community>>,

    #[account(
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == authority.key() @ ErrorCode::InvalidReporter,
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

    /// CHECK: this account is not dangerous
    #[account(
        mut,
        owner = id()
    )]
    pub asset: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfirmAsset<'info> {
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
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        owner = id(),
        has_one = reporter,
        has_one = network,
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump = reporter_reward.bump,
    )]
    pub reporter_reward: Account<'info, ReporterReward>,

    #[account(
        mut,
        owner = id(),
        has_one = network,
        constraint = asset_reporter_reward.reporter == asset.reporter @ ErrorCode::InvalidReporter,
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), asset.reporter.as_ref()],
        bump = asset_reporter_reward.bump,
    )]
    pub asset_reporter_reward: Account<'info, ReporterReward>,

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
        constraint = asset.reporter != reporter.key() @ ErrorCode::Unauthorized,
        has_one = network @ ErrorCode::NetworkMismatch,
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            asset.mint[0..32].as_ref(),
            asset.mint[32..64].as_ref(),
            asset.asset_id.as_ref(),
        ],
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

    /// CHECK: this account is not dangerous
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

#[derive(Accounts)]
pub struct ClaimReporterReward<'info> {
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
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        owner = id(),
        has_one = reporter,
        has_one = network,
        seeds = [b"reporter_reward".as_ref(), network.key().as_ref(), reporter.key().as_ref()],
        bump = reporter_reward.bump,
    )]
    pub reporter_reward: Account<'info, ReporterReward>,

    #[account(
        mut,
        constraint = reporter_token_account.mint == reward_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_token_account.owner == sender.key() @ ProgramError::IllegalOwner,
    )]
    pub reporter_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        owner = Token::id())
    ]
    pub reward_mint: Account<'info, Mint>,

    /// CHECK: this account is not dangerous
    #[account(
        seeds = [b"network_reward".as_ref(), network.key().as_ref()],
        bump = network.reward_signer_bump,
    )]
    pub reward_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct UpdateReplicationPrice<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub community: Box<Account<'info, Community>>,

    #[account(
        mut,
        has_one = community @ ErrorCode::CommunityMismatch,
        seeds = [b"network".as_ref(), community.key().as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Box<Account<'info, Network>>,

    #[account(
        owner = id(),
        has_one = community @ ErrorCode::CommunityMismatch,
        constraint = reporter.role == ReporterRole::Appraiser @ ErrorCode::Unauthorized,
        constraint = reporter.pubkey == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = !reporter.is_frozen @ ErrorCode::FrozenReporter,
        seeds = [b"reporter".as_ref(), community.key().as_ref(), reporter.pubkey.as_ref()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

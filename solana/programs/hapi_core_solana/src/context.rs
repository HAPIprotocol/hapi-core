use {
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};

use crate::{
    error::ErrorCode,
    id,
    program::HapiCoreSolana,
    state::{
        address::*, asset::Asset, case::*, confirmation::*, network::*, reporter::*,
        ACCOUNT_RESERVE_SPACE,
    },
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

    /// CHECK: this account is not dangerous
    #[account(
        constraint = reward_mint.key() == Pubkey::default() || reward_mint.owner == &Token::id() @ ErrorCode::InvalidToken,

    )]
    pub reward_mint: AccountInfo<'info>,

    /// CHECK: this account is not dangerous
    #[account(
        constraint = stake_mint.key() == Pubkey::default() || stake_mint.owner == &Token::id() @ ErrorCode::InvalidToken,

    )]
    pub stake_mint: AccountInfo<'info>,

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
pub struct UpdateStakeConfiguration<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = Token::id(),
        constraint = network.stake_mint == Pubkey::default() || network.stake_mint == stake_mint.key() @ErrorCode::UpdatedMint
    )]
    pub stake_mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct UpdateRewardConfiguration<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ ErrorCode::AuthorityMismatch,
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = Token::id(),
        constraint = network.reward_mint == Pubkey::default() || network.reward_mint == reward_mint.key() @ErrorCode::UpdatedMint
    )]
    pub reward_mint: Account<'info, Mint>,
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
    reporter_id: u128,
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
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter_id.to_be_bytes()],
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
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct ActivateReporter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        mut,
        owner = id(),
        constraint = reporter.status == ReporterStatus::Inactive @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.account == signer.key() @ ErrorCode::InvalidReporter,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        constraint = network_stake_token_account.mint == network.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = network_stake_token_account.owner == network.key() @ ErrorCode::IllegalOwner,
        owner = Token::id(),
    )]
    pub network_stake_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = reporter_stake_token_account.mint == network.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_stake_token_account.owner == signer.key() @ ErrorCode::IllegalOwner,
        owner = Token::id(),
    )]
    pub reporter_stake_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DeactivateReporter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        mut,
        owner = id(),
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.account == signer.key() @ ErrorCode::InvalidReporter,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        mut,
        owner = id(),
        constraint = reporter.status == ReporterStatus::Unstaking @ ErrorCode::InvalidReporterStatus,
        constraint = reporter.account == signer.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.unlock_timestamp <= Clock::get()?.unix_timestamp as u64 @ ErrorCode::ReleaseEpochInFuture,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        constraint = network_stake_token_account.mint == network.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = network_stake_token_account.owner == network.key() @ ErrorCode::IllegalOwner,
        owner = Token::id(),
    )]
    pub network_stake_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = reporter_stake_token_account.mint == network.stake_mint.key() @ ErrorCode::InvalidToken,
        constraint = reporter_stake_token_account.owner == signer.key() @ ErrorCode::IllegalOwner,
        owner = Token::id(),
    )]
    pub reporter_stake_token_account: Account<'info, TokenAccount>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(
    case_id: u128,
    bump: u8,
)]
pub struct CreateCase<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = reporter.role == ReporterRole::Publisher || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"case".as_ref(), network.key().as_ref(), &case_id.to_be_bytes()],
        bump,
        space = Case::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCase<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key()) || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        mut,
        owner = id(),
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(addr: [u8; 64], bump: u8)]
pub struct CreateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = reporter.role == ReporterRole::Tracer
        || reporter.role == ReporterRole::Publisher
        || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
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
        space = Address::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub address: Account<'info, Address>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            address.address[0..32].as_ref(),
            address.address[32..64].as_ref(),
        ],
        bump = address.bump
    )]
    pub address: Account<'info, Address>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]

pub struct ConfirmAddress<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = (address.reporter_id != reporter.id) &&
        (reporter.role == ReporterRole::Validator ||
        reporter.role == ReporterRole::Publisher) @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        constraint = case.id == address.case_id @ ErrorCode::CaseMismatch,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        seeds = [
            b"address".as_ref(),
            network.key().as_ref(),
            address.address[0..32].as_ref(),
            address.address[32..64].as_ref(),
        ],
        bump = address.bump
    )]
    pub address: Account<'info, Address>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"confirmation".as_ref(), address.key().as_ref(), &reporter.id.to_be_bytes()],
        bump,
        space = Confirmation::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub confirmation: Account<'info, Confirmation>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(addr: [u8; 64], asset_id: [u8; 64], bump: u8)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = reporter.role == ReporterRole::Tracer
        || reporter.role == ReporterRole::Publisher
        || reporter.role == ReporterRole::Authority @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        init,
        owner = id(),
        payer = sender,
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            addr[0..32].as_ref(),
            addr[32..64].as_ref(),
            asset_id[0..32].as_ref(),
            asset_id[32..64].as_ref(),
        ],
        bump,
        space = Asset::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub asset: Account<'info, Asset>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = reporter.role == ReporterRole::Authority
            || (reporter.role == ReporterRole::Publisher
            && case.reporter == reporter.key()) @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            asset.address[0..32].as_ref(),
            asset.address[32..64].as_ref(),
            asset.id[0..32].as_ref(),
            asset.id[32..64].as_ref(),
        ],
        bump = asset.bump,
    )]
    pub asset: Account<'info, Asset>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]

pub struct ConfirmAsset<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"network".as_ref(), network.name.as_ref()],
        bump = network.bump,
    )]
    pub network: Account<'info, Network>,

    #[account(
        owner = id(),
        constraint = (asset.reporter_id != reporter.id) &&
            (reporter.role == ReporterRole::Validator ||
            reporter.role == ReporterRole::Publisher) @ ErrorCode::Unauthorized,
        constraint = reporter.account == sender.key() @ ErrorCode::InvalidReporter,
        constraint = reporter.status == ReporterStatus::Active @ ErrorCode::InvalidReporterStatus,
        seeds = [b"reporter".as_ref(), network.key().as_ref(), &reporter.id.to_be_bytes()],
        bump = reporter.bump,
    )]
    pub reporter: Account<'info, Reporter>,

    #[account(
        owner = id(),
        constraint = case.status == CaseStatus::Open @ ErrorCode::CaseClosed,
        constraint = case.id == asset.case_id @ ErrorCode::CaseMismatch,
        seeds = [b"case".as_ref(), network.key().as_ref(), &case.id.to_be_bytes()],
        bump = case.bump,
    )]
    pub case: Account<'info, Case>,

    #[account(
        mut,
        owner = id(),
        seeds = [
            b"asset".as_ref(),
            network.key().as_ref(),
            asset.address[0..32].as_ref(),
            asset.address[32..64].as_ref(),
            asset.id[0..32].as_ref(),
            asset.id[32..64].as_ref(),
        ],
        bump = asset.bump,
    )]
    pub asset: Account<'info, Asset>,

    #[account(
        init,
        payer = sender,
        owner = id(),
        seeds = [b"confirmation".as_ref(), asset.key().as_ref(), &reporter.id.to_be_bytes()],
        bump,
        space = Confirmation::LEN + ACCOUNT_RESERVE_SPACE
    )]
    pub confirmation: Account<'info, Confirmation>,

    pub system_program: Program<'info, System>,
}

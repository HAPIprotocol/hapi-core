use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, SetAuthority, Transfer};
use spl_token::instruction::AuthorityType;
use std::io::Write;

declare_id!("8DCgGWyLHPsESt5EgPG2asnxhhC7P3f8ZoK4zZ93hoQE");

pub mod checker;
pub mod context;
pub mod error;
pub mod state;

use context::*;
use error::{print_error, ErrorCode};
use state::{
    address::DeprecatedAddress,
    asset::{Asset, DeprecatedAsset},
    community::{Community, DeprecatedCommunity},
    network::{DeprecatedNetwork, Network},
};
pub use state::{
    address::{Address, Category},
    case::CaseStatus,
    network::NetworkSchema,
    reporter::{ReporterRole, ReporterStatus},
};

fn realloc_and_rent<'info>(
    account: &AccountInfo<'info>,
    payer: &Signer<'info>,
    rent: &Sysvar<'info, Rent>,
    len: usize,
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    // Realloc
    account.realloc(len, false)?;

    let balance = account.lamports();
    if rent.is_exempt(balance, len) {
        return Ok(());
    }

    // Transfer some lamports
    let min_balance = rent.minimum_balance(len);
    if balance.ge(&min_balance) {
        return Ok(());
    }

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &payer.key(),
        &account.key(),
        min_balance - balance,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[payer.to_account_info(), account.to_account_info()],
    )
}

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize_community(
        ctx: Context<InitializeCommunity>,
        stake_unlock_epochs: u64,
        confirmation_threshold: u8,
        validator_stake: u64,
        tracer_stake: u64,
        full_stake: u64,
        authority_stake: u64,
        appraiser_stake: u64,
        signer_bump: u8,
    ) -> Result<()> {
        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.authority.key;
        community.cases = 0;
        community.stake_unlock_epochs = stake_unlock_epochs;
        community.confirmation_threshold = confirmation_threshold;
        community.stake_mint = ctx.accounts.stake_mint.to_account_info().key();
        community.token_signer = ctx.accounts.token_signer.key();
        community.token_signer_bump = signer_bump;
        community.token_account = ctx.accounts.token_account.key();
        community.treasury_token_account = ctx.accounts.treasury_token_account.key();
        community.validator_stake = validator_stake;
        community.tracer_stake = tracer_stake;
        community.full_stake = full_stake;
        community.authority_stake = authority_stake;
        community.appraiser_stake = appraiser_stake;

        Ok(())
    }

    pub fn update_community(
        ctx: Context<UpdateCommunity>,
        stake_unlock_epochs: u64,
        confirmation_threshold: u8,
        validator_stake: u64,
        tracer_stake: u64,
        full_stake: u64,
        authority_stake: u64,
        appraiser_stake: u64,
    ) -> Result<()> {
        let community = &mut ctx.accounts.community;

        community.stake_unlock_epochs = stake_unlock_epochs;
        community.confirmation_threshold = confirmation_threshold;
        community.validator_stake = validator_stake;
        community.tracer_stake = tracer_stake;
        community.full_stake = full_stake;
        community.authority_stake = authority_stake;
        community.appraiser_stake = appraiser_stake;

        Ok(())
    }

    pub fn migrate_community(ctx: Context<MigrateCommunity>, appraiser_stake: u64) -> Result<()> {
        let deprecated_community = DeprecatedCommunity::try_deserialize_unchecked(
            &mut ctx.accounts.community.try_borrow_data()?.as_ref(),
        )?;

        if deprecated_community.authority != ctx.accounts.authority.key() {
            return print_error(ErrorCode::NetworkMismatch);
        }

        let community = Community::from_deprecated(
            deprecated_community,
            ctx.accounts.treasury_token_account.key(),
            appraiser_stake,
        );
        let community_size = std::mem::size_of::<Community>();

        let mut buffer: Vec<u8> = Vec::new();
        community.try_serialize(&mut buffer)?;

        if buffer.len() != community_size {
            return print_error(ErrorCode::AccountDidNotSerialize);
        }

        realloc_and_rent(
            &ctx.accounts.community,
            &ctx.accounts.authority,
            &ctx.accounts.rent,
            community_size + 25,
        )?;
        ctx.accounts
            .community
            .try_borrow_mut_data()?
            .write_all(&buffer)?;

        Ok(())
    }

    pub fn set_community_authority(ctx: Context<SetCommunityAuthority>) -> Result<()> {
        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.new_authority.key;

        Ok(())
    }

    pub fn create_network(
        ctx: Context<CreateNetwork>,
        name: [u8; 32],
        schema: NetworkSchema,
        address_tracer_reward: u64,
        address_confirmation_reward: u64,
        asset_tracer_reward: u64,
        asset_confirmation_reward: u64,
        network_bump: u8,
        reward_signer_bump: u8,
        report_price: u64,
    ) -> Result<()> {
        // Pass authority to network signer PDA
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.authority.to_account_info(),
                    account_or_mint: ctx.accounts.reward_mint.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            Some(ctx.accounts.reward_signer.key()),
        )?;

        let network = &mut ctx.accounts.network;

        network.community = ctx.accounts.community.key();
        network.bump = network_bump;

        network.name = name;
        network.schema = schema;
        network.reward_mint = ctx.accounts.reward_mint.key();
        network.reward_signer = ctx.accounts.reward_signer.key();
        network.reward_signer_bump = reward_signer_bump;
        network.address_tracer_reward = address_tracer_reward;
        network.address_confirmation_reward = address_confirmation_reward;
        network.asset_tracer_reward = asset_tracer_reward;
        network.asset_confirmation_reward = asset_confirmation_reward;
        network.replication_price = report_price;

        Ok(())
    }

    pub fn update_network(
        ctx: Context<UpdateNetwork>,
        address_tracer_reward: u64,
        address_confirmation_reward: u64,
        asset_tracer_reward: u64,
        asset_confirmation_reward: u64,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.address_tracer_reward = address_tracer_reward;
        network.address_confirmation_reward = address_confirmation_reward;
        network.asset_tracer_reward = asset_tracer_reward;
        network.asset_confirmation_reward = asset_confirmation_reward;

        Ok(())
    }

    pub fn migrate_network(ctx: Context<MigrateNetwork>) -> Result<()> {
        let deprecated_network = DeprecatedNetwork::try_deserialize_unchecked(
            &mut ctx.accounts.network.try_borrow_data()?.as_ref(),
        )?;

        let (pda, bump) = Pubkey::find_program_address(
            &[
                b"network".as_ref(),
                ctx.accounts.community.key().as_ref(),
                deprecated_network.name.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.network.key() == pda && deprecated_network.bump == bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }
        if deprecated_network.community != ctx.accounts.community.key() {
            return print_error(ErrorCode::CommunityMismatch);
        }

        let network = Network::from_deprecated(deprecated_network);
        let network_size = std::mem::size_of::<Network>();

        let mut buffer: Vec<u8> = Vec::new();
        network.try_serialize(&mut buffer)?;

        if buffer.len() != network_size {
            return print_error(ErrorCode::AccountDidNotSerialize);
        }

        realloc_and_rent(
            &ctx.accounts.network,
            &ctx.accounts.authority,
            &ctx.accounts.rent,
            network_size,
        )?;
        ctx.accounts
            .network
            .try_borrow_mut_data()?
            .write_all(&buffer)?;

        Ok(())
    }

    pub fn create_reporter(
        ctx: Context<CreateReporter>,
        role: ReporterRole,
        name: [u8; 32],
        bump: u8,
    ) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.community = ctx.accounts.community.key();
        reporter.pubkey = *ctx.accounts.pubkey.key;
        reporter.bump = bump;

        reporter.role = role;
        reporter.status = ReporterStatus::Inactive;
        reporter.name = name;
        reporter.is_frozen = false;
        reporter.stake = 0;

        Ok(())
    }

    pub fn update_reporter(
        ctx: Context<UpdateReporter>,
        role: ReporterRole,
        name: [u8; 32],
    ) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.role = role;
        reporter.name = name;

        Ok(())
    }

    pub fn create_case(
        ctx: Context<CreateCase>,
        case_id: u64,
        name: [u8; 32],
        bump: u8,
    ) -> Result<()> {
        let community = &mut ctx.accounts.community;

        if case_id != community.cases + 1 {
            return print_error(ErrorCode::NonSequentialCaseId);
        } else {
            community.cases = case_id;
        }

        let case = &mut ctx.accounts.case;

        case.community = ctx.accounts.community.key();
        case.id = case_id;
        case.bump = bump;

        case.name = name;
        case.status = CaseStatus::Open;
        case.reporter = ctx.accounts.reporter.key();

        Ok(())
    }

    pub fn update_case(ctx: Context<UpdateCase>, name: [u8; 32], status: CaseStatus) -> Result<()> {
        let case = &mut ctx.accounts.case;

        case.name = name;
        case.status = status;

        Ok(())
    }

    pub fn create_address(
        ctx: Context<CreateAddress>,
        addr: [u8; 64],
        category: Category,
        risk: u8,
        bump: u8,
    ) -> Result<()> {
        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx
                        .accounts
                        .reporter_payment_token_account
                        .to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            ctx.accounts.network.replication_price,
        )?;

        let address = &mut ctx.accounts.address;

        address.network = ctx.accounts.network.key();
        address.address = addr;
        address.bump = bump;

        address.community = ctx.accounts.community.key();
        address.reporter = ctx.accounts.reporter.key();
        address.case_id = ctx.accounts.case.id;
        address.category = category;
        address.risk = risk;
        address.confirmations = 0;
        address.replication_bounty = ctx.accounts.network.replication_price;

        Ok(())
    }

    pub fn confirm_address(ctx: Context<ConfirmAddress>) -> Result<()> {
        let address = &mut ctx.accounts.address;

        address.confirmations += 1;

        let community = &ctx.accounts.community;

        if address.confirmations == community.confirmation_threshold {
            let address_reporter_reward = &mut ctx.accounts.address_reporter_reward.load_mut()?;

            address_reporter_reward.address_tracer_counter += 1;
        }

        let reporter_reward = &mut ctx.accounts.reporter_reward.load_mut()?;

        reporter_reward.address_confirmation_counter += 1;

        Ok(())
    }

    pub fn update_address(ctx: Context<UpdateAddress>, category: Category, risk: u8) -> Result<()> {
        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx
                        .accounts
                        .reporter_payment_token_account
                        .to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            ctx.accounts.network.replication_price,
        )?;

        let address = &mut ctx.accounts.address;

        address.risk = risk;
        address.category = category;
        address.replication_bounty = address
            .replication_bounty
            .checked_add(ctx.accounts.network.replication_price)
            .unwrap();

        Ok(())
    }

    pub fn migrate_address(ctx: Context<MigrateAddress>) -> Result<()> {
        let deprecated_address = DeprecatedAddress::try_deserialize_unchecked(
            &mut ctx.accounts.address.try_borrow_data()?.as_ref(),
        )?;

        let (pda, bump) = Pubkey::find_program_address(
            &[
                b"address".as_ref(),
                ctx.accounts.network.key().as_ref(),
                deprecated_address.address[0..32].as_ref(),
                deprecated_address.address[32..64].as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.address.key() == pda && deprecated_address.bump == bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }
        if deprecated_address.case_id != ctx.accounts.case.id {
            return print_error(ErrorCode::CaseMismatch);
        }
        if deprecated_address.network != ctx.accounts.network.key() {
            return print_error(ErrorCode::NetworkMismatch);
        }

        let address = Address::from_deprecated(deprecated_address);
        let address_size = std::mem::size_of::<Address>();

        let mut buffer: Vec<u8> = Vec::new();
        address.try_serialize(&mut buffer)?;

        if buffer.len() != address_size {
            return print_error(ErrorCode::AccountDidNotSerialize);
        }

        realloc_and_rent(
            &ctx.accounts.address,
            &ctx.accounts.authority,
            &ctx.accounts.rent,
            address_size,
        )?;
        ctx.accounts
            .address
            .try_borrow_mut_data()?
            .write_all(&buffer)?;

        Ok(())
    }

    pub fn change_address_case(ctx: Context<ChangeAddressCase>) -> Result<()> {
        let address = &mut ctx.accounts.address;

        address.case_id = ctx.accounts.new_case.id;

        Ok(())
    }

    pub fn create_asset(
        ctx: Context<CreateAsset>,
        mint: [u8; 64],
        asset_id: [u8; 32],
        category: Category,
        risk: u8,
        bump: u8,
    ) -> Result<()> {
        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx
                        .accounts
                        .reporter_payment_token_account
                        .to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            ctx.accounts.network.replication_price,
        )?;

        let asset = &mut ctx.accounts.asset;

        asset.network = ctx.accounts.network.key();
        asset.mint = mint;
        asset.asset_id = asset_id;
        asset.bump = bump;

        asset.community = ctx.accounts.community.key();
        asset.reporter = ctx.accounts.reporter.key();
        asset.case_id = ctx.accounts.case.id;
        asset.category = category;
        asset.risk = risk;
        asset.confirmations = 0;
        asset.replication_bounty = ctx.accounts.network.replication_price;

        Ok(())
    }

    pub fn confirm_asset(ctx: Context<ConfirmAsset>) -> Result<()> {
        let asset = &mut ctx.accounts.asset;

        asset.confirmations += 1;

        let community = &ctx.accounts.community;

        if asset.confirmations == community.confirmation_threshold {
            let asset_reporter_reward = &mut ctx.accounts.asset_reporter_reward.load_mut()?;

            asset_reporter_reward.asset_tracer_counter += 1;
        }

        let reporter_reward = &mut ctx.accounts.reporter_reward.load_mut()?;

        reporter_reward.asset_confirmation_counter += 1;

        Ok(())
    }

    pub fn update_asset(ctx: Context<UpdateAsset>, category: Category, risk: u8) -> Result<()> {
        if risk > 10 {
            return print_error(ErrorCode::RiskOutOfRange);
        }

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx
                        .accounts
                        .reporter_payment_token_account
                        .to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            ctx.accounts.network.replication_price,
        )?;

        let asset = &mut ctx.accounts.asset;

        asset.risk = risk;
        asset.category = category;
        asset.replication_bounty = asset
            .replication_bounty
            .checked_add(ctx.accounts.network.replication_price)
            .unwrap();

        Ok(())
    }

    pub fn migrate_asset(ctx: Context<MigrateAsset>) -> Result<()> {
        let deprecated_asset = DeprecatedAsset::try_deserialize_unchecked(
            &mut ctx.accounts.asset.try_borrow_data()?.as_ref(),
        )?;

        let (pda, bump) = Pubkey::find_program_address(
            &[
                b"asset".as_ref(),
                ctx.accounts.network.key().as_ref(),
                deprecated_asset.mint[0..32].as_ref(),
                deprecated_asset.mint[32..64].as_ref(),
                deprecated_asset.asset_id.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.asset.key() == pda && deprecated_asset.bump == bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }
        if deprecated_asset.case_id != ctx.accounts.case.id {
            return print_error(ErrorCode::CaseMismatch);
        }
        if deprecated_asset.network != ctx.accounts.network.key() {
            return print_error(ErrorCode::NetworkMismatch);
        }

        let asset = Asset::from_deprecated(deprecated_asset);
        let asset_size = std::mem::size_of::<Asset>();

        let mut buffer: Vec<u8> = Vec::new();
        asset.try_serialize(&mut buffer)?;

        if buffer.len() != asset_size {
            return print_error(ErrorCode::AccountDidNotSerialize);
        }

        realloc_and_rent(
            &ctx.accounts.asset,
            &ctx.accounts.authority,
            &ctx.accounts.rent,
            asset_size,
        )?;
        ctx.accounts
            .asset
            .try_borrow_mut_data()?
            .write_all(&buffer)?;

        Ok(())
    }

    pub fn initialize_reporter_reward(
        ctx: Context<InitializeReporterReward>,
        bump: u8,
    ) -> Result<()> {
        let reporter_reward = &mut ctx.accounts.reporter_reward.load_init()?;

        reporter_reward.network = ctx.accounts.network.key();
        reporter_reward.reporter = ctx.accounts.reporter.key();
        reporter_reward.bump = bump;

        Ok(())
    }

    pub fn activate_reporter(ctx: Context<ActivateReporter>) -> Result<()> {
        let community = &ctx.accounts.community;

        let reporter = &mut ctx.accounts.reporter;

        let stake = match reporter.role {
            ReporterRole::Validator => community.validator_stake,
            ReporterRole::Tracer => community.tracer_stake,
            ReporterRole::Publisher => community.full_stake,
            ReporterRole::Authority => community.authority_stake,
            ReporterRole::Appraiser => community.appraiser_stake,
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.reporter_token_account.to_account_info(),
                    to: ctx.accounts.community_token_account.to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            stake,
        )?;

        reporter.status = ReporterStatus::Active;
        reporter.stake = stake;

        Ok(())
    }

    pub fn deactivate_reporter(ctx: Context<DeactivateReporter>) -> Result<()> {
        let community = &ctx.accounts.community;

        let reporter = &mut ctx.accounts.reporter;

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_epoch = Clock::get()?.epoch + community.stake_unlock_epochs;

        Ok(())
    }

    pub fn release_reporter(ctx: Context<ReleaseReporter>) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        if reporter.unlock_epoch > Clock::get()?.epoch {
            return print_error(ErrorCode::ReleaseEpochInFuture);
        }

        let community = ctx.accounts.community.clone();

        let token_signer = ctx.accounts.community_token_signer.clone();

        let seeds = &[
            b"community_stash".as_ref(),
            community.to_account_info().key.as_ref(),
            &[community.token_signer_bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.community_token_account.to_account_info(),
                to: ctx.accounts.reporter_token_account.to_account_info(),
                authority: token_signer.to_account_info(),
            },
            signer,
        );

        token::transfer(cpi_context, reporter.stake)?;

        reporter.status = ReporterStatus::Inactive;
        reporter.unlock_epoch = 0;
        reporter.stake = 0;

        Ok(())
    }

    pub fn claim_reporter_reward(ctx: Context<ClaimReporterReward>) -> Result<()> {
        let network = &ctx.accounts.network;

        let reporter_reward = &mut ctx.accounts.reporter_reward.load_mut()?;

        let reward = network.address_confirmation_reward
            * reporter_reward.address_confirmation_counter as u64
            + network.address_tracer_reward * reporter_reward.address_tracer_counter as u64;

        if reward == 0 {
            return print_error(ErrorCode::NoReward);
        }

        reporter_reward.address_confirmation_counter = 0;
        reporter_reward.address_tracer_counter = 0;

        let reward_signer = &ctx.accounts.reward_signer;

        let seeds = &[
            b"network_reward",
            network.to_account_info().key.as_ref(),
            &[network.reward_signer_bump],
        ];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.reward_mint.to_account_info(),
                    to: ctx.accounts.reporter_token_account.to_account_info(),
                    authority: reward_signer.to_account_info(),
                },
                &[&seeds[..]],
            ),
            reward,
        )?;

        Ok(())
    }

    pub fn freeze_reporter(ctx: Context<FreezeReporter>) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.is_frozen = true;

        Ok(())
    }

    pub fn unfreeze_reporter(ctx: Context<UnfreezeReporter>) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;

        reporter.is_frozen = false;

        Ok(())
    }

    pub fn update_replication_price(
        ctx: Context<UpdateReplicationPrice>,
        price: u64,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        network.replication_price = price;

        Ok(())
    }
}

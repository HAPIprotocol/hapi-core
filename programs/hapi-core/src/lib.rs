use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, MintTo, SetAuthority, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("hapiAwBQLYRXrjGn6FLCgC8FpQd2yWbKMqS6AYZ48g6");

pub mod checker;
pub mod context;
pub mod error;
pub mod state;
pub mod utils;

use context::*;
use error::{print_error, ErrorCode};
use state::{
    asset::Asset,
    case::Case,
    community::Community,
    network::Network,
    reporter::{Reporter, ReporterReward},
};
use utils::close;

pub use state::{
    address::{Address, Category},
    case::CaseStatus,
    network::NetworkSchema,
    reporter::{ReporterRole, ReporterStatus},
};

#[program]
pub mod hapi_core {
    use super::*;

    pub fn initialize_community(
        ctx: Context<InitializeCommunity>,
        community_id: u64,
        bump: u8,
        stake_unlock_epochs: u64,
        confirmation_threshold: u8,
        validator_stake: u64,
        tracer_stake: u64,
        full_stake: u64,
        authority_stake: u64,
        appraiser_stake: u64,
    ) -> Result<()> {
        let community = &mut ctx.accounts.community;

        community.authority = *ctx.accounts.authority.key;
        community.community_id = community_id;
        community.bump = bump;
        community.cases = 0;
        community.stake_unlock_epochs = stake_unlock_epochs;
        community.confirmation_threshold = confirmation_threshold;
        community.stake_mint = ctx.accounts.stake_mint.to_account_info().key();
        community.validator_stake = validator_stake;
        community.tracer_stake = tracer_stake;
        community.full_stake = full_stake;
        community.authority_stake = authority_stake;
        community.appraiser_stake = appraiser_stake;
        community.version = Community::VERSION;

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

    pub fn migrate_community(
        ctx: Context<MigrateCommunity>,
        community_id: u64,
        bump: u8,
        token_signer_bump: u8,
    ) -> Result<()> {
        let community_data = Community::from_deprecated(
            &mut ctx.accounts.old_community.try_borrow_data()?.as_ref(),
        )?;

        if community_data.authority != ctx.accounts.authority.key() {
            return print_error(ErrorCode::AuthorityMismatch);
        }

        let seeds = &[
            b"community_stash".as_ref(),
            ctx.accounts.old_community.to_account_info().key.as_ref(),
            &[token_signer_bump],
        ];
        let signer = &[&seeds[..]];

        // Initializing new account
        let community = &mut ctx.accounts.community;
        community.set_inner(community_data);
        community.community_id = community_id;
        community.bump = bump;

        // Transfer all tokens to new ATA
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.old_token_account.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.token_signer.to_account_info(),
                },
                signer,
            ),
            ctx.accounts.old_token_account.amount,
        )?;

        // Close old token account
        token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.old_token_account.to_account_info(),
                destination: ctx.accounts.authority.to_account_info(),
                authority: ctx.accounts.token_signer.to_account_info(),
            },
            signer,
        ))?;

        // Closing old community account
        close(
            ctx.accounts.old_community.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
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
        report_price: u64,
    ) -> Result<()> {
        let network = &mut ctx.accounts.network;

        // Pass authority to network PDA
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.authority.to_account_info(),
                    account_or_mint: ctx.accounts.reward_mint.to_account_info(),
                },
            ),
            AuthorityType::MintTokens,
            Some(network.key()),
        )?;

        network.community = ctx.accounts.community.key();
        network.bump = network_bump;
        network.name = name;
        network.schema = schema;
        network.reward_mint = ctx.accounts.reward_mint.key();
        network.address_tracer_reward = address_tracer_reward;
        network.address_confirmation_reward = address_confirmation_reward;
        network.asset_tracer_reward = asset_tracer_reward;
        network.asset_confirmation_reward = asset_confirmation_reward;
        network.replication_price = report_price;
        network.version = Network::VERSION;

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

    pub fn migrate_network(
        ctx: Context<MigrateNetwork>,
        name: [u8; 32],
        bump: u8,
        reward_signer_bump: u8,
    ) -> Result<()> {
        let network_data =
            Network::from_deprecated(&mut ctx.accounts.old_network.try_borrow_data()?.as_ref())?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"network".as_ref(),
                network_data.community.as_ref(),
                network_data.name.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.old_network.key() != pda
            || network_data.bump != old_bump
            || network_data.name != name
        {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        let seeds = &[
            b"network_reward".as_ref(),
            ctx.accounts.old_network.to_account_info().key.as_ref(),
            &[reward_signer_bump],
        ];

        // Initializing new account
        let network = &mut ctx.accounts.network;
        network.set_inner(network_data);
        network.community = ctx.accounts.community.key();
        network.bump = bump;

        // Set reward mint authority to network
        token::set_authority(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.reward_signer.to_account_info(),
                    account_or_mint: ctx.accounts.reward_mint.to_account_info(),
                },
                &[&seeds[..]],
            ),
            AuthorityType::MintTokens,
            Some(network.key()),
        )?;

        // Closing old network account
        close(
            ctx.accounts.old_network.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
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
        reporter.version = Reporter::VERSION;

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

    pub fn migrate_reporter(ctx: Context<MigrateReporter>, bump: u8) -> Result<()> {
        let reporter_data =
            Reporter::from_deprecated(&mut ctx.accounts.old_reporter.try_borrow_data()?.as_ref())?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"reporter".as_ref(),
                reporter_data.community.as_ref(),
                reporter_data.pubkey.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.old_reporter.key() != pda || reporter_data.bump != old_bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        // Initializing new account
        let reporter = &mut ctx.accounts.reporter;
        reporter.set_inner(reporter_data);
        reporter.community = ctx.accounts.community.key();
        reporter.bump = bump;

        // Closing old reporter account
        close(
            ctx.accounts.old_reporter.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
    }

    pub fn migrate_reporter_reward(ctx: Context<MigrateReporterReward>, bump: u8) -> Result<()> {
        let reporter_reward_data = ReporterReward::from_deprecated(
            &mut ctx.accounts.old_reporter_reward.try_borrow_data()?.as_ref(),
        )?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"reporter_reward".as_ref(),
                reporter_reward_data.network.as_ref(),
                reporter_reward_data.reporter.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.old_reporter_reward.key() != pda || reporter_reward_data.bump != old_bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        // Initializing new account
        let reporter_reward = &mut ctx.accounts.reporter_reward;
        reporter_reward.set_inner(reporter_reward_data);
        reporter_reward.network = ctx.accounts.network.key();
        reporter_reward.reporter = ctx.accounts.reporter.key();
        reporter_reward.bump = bump;

        // Closing old reporter reward account
        close(
            ctx.accounts.old_reporter_reward.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
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
        case.version = Case::VERSION;

        Ok(())
    }

    pub fn update_case(ctx: Context<UpdateCase>, name: [u8; 32], status: CaseStatus) -> Result<()> {
        let case = &mut ctx.accounts.case;

        case.name = name;
        case.status = status;

        Ok(())
    }

    pub fn migrate_case(ctx: Context<MigrateCase>, case_id: u64, bump: u8) -> Result<()> {
        let case_data =
            Case::from_deprecated(&mut ctx.accounts.old_case.try_borrow_data()?.as_ref())?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"case".as_ref(),
                case_data.community.as_ref(),
                &case_data.id.to_le_bytes(),
            ],
            &id(),
        );

        if ctx.accounts.old_case.key() != pda
            || case_data.bump != old_bump
            || case_data.id != case_id
        {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        // Initializing new account
        let case = &mut ctx.accounts.case;
        case.set_inner(case_data);
        case.community = ctx.accounts.community.key();
        case.reporter = ctx.accounts.reporter.key();
        case.bump = bump;

        // Closing old case account
        close(
            ctx.accounts.old_case.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
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
        address.version = Address::VERSION;

        Ok(())
    }

    pub fn confirm_address(ctx: Context<ConfirmAddress>) -> Result<()> {
        let address = &mut ctx.accounts.address;

        address.confirmations += 1;

        let community = &ctx.accounts.community;

        if address.confirmations == community.confirmation_threshold {
            let address_reporter_reward = &mut ctx.accounts.address_reporter_reward;

            address_reporter_reward.address_tracer_counter += 1;
        }

        let reporter_reward = &mut ctx.accounts.reporter_reward;

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

    pub fn migrate_address(ctx: Context<MigrateAddress>, addr: [u8; 64], bump: u8) -> Result<()> {
        let address_data =
            Address::from_deprecated(&mut ctx.accounts.old_address.try_borrow_data()?.as_ref())?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"address".as_ref(),
                address_data.network.as_ref(),
                addr[0..32].as_ref(),
                addr[32..64].as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.old_address.key() != pda
            || address_data.bump != old_bump
            || address_data.address != addr
        {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        // Initializing new account
        let address = &mut ctx.accounts.address;
        address.set_inner(address_data);
        address.community = ctx.accounts.community.key();
        address.network = ctx.accounts.network.key();
        address.reporter = ctx.accounts.reporter.key();
        address.bump = bump;

        // Closing old address account
        close(
            ctx.accounts.old_address.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
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
        asset.version = Asset::VERSION;

        Ok(())
    }

    pub fn confirm_asset(ctx: Context<ConfirmAsset>) -> Result<()> {
        let asset = &mut ctx.accounts.asset;

        asset.confirmations += 1;

        let community = &ctx.accounts.community;

        if asset.confirmations == community.confirmation_threshold {
            let asset_reporter_reward = &mut ctx.accounts.asset_reporter_reward;

            asset_reporter_reward.asset_tracer_counter += 1;
        }

        let reporter_reward = &mut ctx.accounts.reporter_reward;

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

    pub fn migrate_asset(
        ctx: Context<MigrateAsset>,
        mint: [u8; 64],
        asset_id: [u8; 32],
        bump: u8,
    ) -> Result<()> {
        let asset_data =
            Asset::from_deprecated(&mut ctx.accounts.old_asset.try_borrow_data()?.as_ref())?;

        let (pda, old_bump) = Pubkey::find_program_address(
            &[
                b"asset".as_ref(),
                asset_data.network.key().as_ref(),
                asset_data.mint[0..32].as_ref(),
                asset_data.mint[32..64].as_ref(),
                asset_data.asset_id.as_ref(),
            ],
            &id(),
        );

        if ctx.accounts.old_asset.key() == pda && asset_data.bump == old_bump
            || asset_data.asset_id != asset_id
            || asset_data.mint != mint
        {
            return print_error(ErrorCode::UnexpectedAccount);
        }

        // Initializing new account
        let asset = &mut ctx.accounts.asset;
        asset.set_inner(asset_data);
        asset.community = ctx.accounts.community.key();
        asset.network = ctx.accounts.network.key();
        asset.reporter = ctx.accounts.reporter.key();
        asset.bump = bump;

        // Closing old asset account
        close(
            ctx.accounts.old_asset.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        )
    }

    pub fn initialize_reporter_reward(
        ctx: Context<InitializeReporterReward>,
        bump: u8,
    ) -> Result<()> {
        let reporter_reward = &mut ctx.accounts.reporter_reward;

        reporter_reward.network = ctx.accounts.network.key();
        reporter_reward.reporter = ctx.accounts.reporter.key();
        reporter_reward.bump = bump;
        reporter_reward.version = ReporterReward::VERSION;

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

        let seeds = &[
            b"community".as_ref(),
            &community.community_id.to_le_bytes(),
            &[community.bump],
        ];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.community_token_account.to_account_info(),
                    to: ctx.accounts.reporter_token_account.to_account_info(),
                    authority: community.to_account_info(),
                },
                &[&seeds[..]],
            ),
            reporter.stake,
        )?;

        reporter.status = ReporterStatus::Inactive;
        reporter.unlock_epoch = 0;
        reporter.stake = 0;

        Ok(())
    }

    pub fn claim_reporter_reward(ctx: Context<ClaimReporterReward>) -> Result<()> {
        let network = &ctx.accounts.network;

        let reporter_reward = &mut ctx.accounts.reporter_reward;

        let reward = network.address_confirmation_reward
            * reporter_reward.address_confirmation_counter as u64
            + network.address_tracer_reward * reporter_reward.address_tracer_counter as u64;

        if reward == 0 {
            return print_error(ErrorCode::NoReward);
        }

        reporter_reward.address_confirmation_counter = 0;
        reporter_reward.address_tracer_counter = 0;

        let signer = &[
            b"network",
            network.community.as_ref(),
            network.name.as_ref(),
            &[network.bump],
        ];

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.reward_mint.to_account_info(),
                    to: ctx.accounts.reporter_token_account.to_account_info(),
                    authority: ctx.accounts.network.to_account_info(),
                },
                &[&signer[..]],
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

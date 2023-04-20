use crate::configuration::{CommunityCfg, HapiCfg};

use {
    anchor_client::{
        anchor_lang::{AccountDeserialize, Discriminator},
        solana_sdk::{
            commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        },
        Client, Cluster, Program,
    },
    anyhow::{Error, Result},
    colored::*,
    hapi_core::{
        accounts, id, instruction,
        state::{
            address::Address,
            asset::Asset,
            case::Case,
            community::Community,
            deprecated::{
                deprecated_address::AddressV0, deprecated_asset::AssetV0, deprecated_case::CaseV0,
                deprecated_community::CommunityV0, deprecated_network::NetworkV0,
                deprecated_reporter::ReporterV0, deprecated_reporter_reward::ReporterRewardV0,
            },
            network::Network,
            reporter::{Reporter, ReporterReward},
        },
    },
    spl_associated_token_account::{
        get_associated_token_address,
        instruction::create_associated_token_account,
        solana_program::{system_program, sysvar::rent},
    },
    std::{rc::Rc, str::FromStr},
};

/// Returns rpc client
pub fn get_program(cfg: &HapiCfg) -> Result<Program> {
    let payer =
        read_keypair_file(cfg.keypair_path.clone()).map_err(|err| Error::msg(err.to_string()))?;
    let environment = Cluster::from_str(&cfg.environment)?;
    let program_id = if !cfg.program_id.is_empty() {
        cfg.program_id.parse::<Pubkey>()?
    } else {
        id()
    };

    let client =
        Client::new_with_options(environment, Rc::new(payer), CommitmentConfig::processed());

    Ok(client.program(program_id))
}

pub struct HapiCli {
    cli: Program,
    communities_cfg: Vec<CommunityCfg>,
}

impl HapiCli {
    pub fn new(cfg: &HapiCfg) -> Result<Self> {
        Ok(Self {
            cli: get_program(cfg)?,
            communities_cfg: cfg.communities.clone(),
        })
    }

    fn get_program_accounts_with_discriminator<T: AccountDeserialize>(
        &self,
        discriminator: [u8; 8],
    ) -> Result<Vec<(Pubkey, T)>> {
        let mut accounts = vec![];

        let results = self
            .cli
            .rpc()
            .get_program_accounts(&self.cli.id())?
            .into_iter();

        for (key, account) in results {
            if account.data.len() >= 8 && account.data[..8] == discriminator {
                if let Ok(acc) = T::try_deserialize_unchecked(&mut (&account.data as &[u8])) {
                    accounts.push((key, acc));
                }
            }
        }

        Ok(accounts)
    }

    fn match_community(&self, pk: &Pubkey) -> bool {
        self.communities_cfg
            .iter()
            .any(|cfg| cfg.pubkey == pk.to_string())
    }

    pub fn migrate_communities(&self) -> Result<()> {
        let mut communities = self
            .get_program_accounts_with_discriminator::<CommunityV0>(Community::discriminator())?;

        communities.retain(|(pk, _)| self.match_community(pk));

        if communities.is_empty() {
            println!(
                "{}",
                "This program has no communities to migrate\n".yellow()
            );
        } else {
            println!("Starting migration of {} communities", communities.len());

            for (pk, community) in communities {
                println!("Migrating community: {}", pk);

                // TODO: remove
                let cfg = self
                    .communities_cfg
                    .iter()
                    .find(|cfg| cfg.pubkey == pk.to_string())
                    .ok_or_else(|| {
                        anyhow::Error::msg(format!("Community {} is absent in config", pk))
                    })?;

                self.cli
                    .request()
                    .instruction(create_associated_token_account(
                        &self.cli.payer(),
                        &pk,
                        &community.stake_mint,
                        &spl_token::ID,
                    ))
                    .send()?;

                let token_account = get_associated_token_address(&pk, &community.stake_mint);

                println!("New community ATA: {}", token_account);

                let signer = read_keypair_file(cfg.keypair_path.clone())
                    .map_err(|err| Error::msg(err.to_string()))?;

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCommunity {
                        authority: self.cli.payer(),
                        community: pk,
                        stake_mint: community.stake_mint,
                        token_signer: community.token_signer,
                        old_token_account: community.token_account,
                        token_account,
                        rent: rent::ID,
                        token_program: spl_token::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCommunity {
                        token_signer_bump: community.token_signer_bump,
                    })
                    .signer(&signer)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All communities migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_networks(&self) -> Result<()> {
        let mut networks =
            self.get_program_accounts_with_discriminator::<NetworkV0>(Network::discriminator())?;

        networks.retain(|(_, old_acc)| self.match_community(&old_acc.community));

        if networks.is_empty() {
            println!("{}", "This program has no networks to migrate\n".yellow());
        } else {
            println!("Starting migration of {} networks", networks.len());

            for (pk, network) in networks {
                println!("Migrating network: {}", pk);

                self.cli
                    .request()
                    .instruction(create_associated_token_account(
                        &self.cli.payer(),
                        &pk,
                        &network.reward_mint,
                        &spl_token::ID,
                    ))
                    .send()?;

                let treasury_token_account =
                    get_associated_token_address(&pk, &network.reward_mint);

                println!("Network treasury ATA: {}", treasury_token_account);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateNetwork {
                        authority: self.cli.payer(),
                        community: network.community,
                        network: pk,
                        reward_signer: network.reward_signer,
                        reward_mint: network.reward_mint,
                        treasury_token_account,
                        rent: rent::ID,
                        token_program: spl_token::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateNetwork {
                        reward_signer_bump: network.reward_signer_bump,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All networks migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_reporters(&self) -> Result<()> {
        let mut reporters =
            self.get_program_accounts_with_discriminator::<ReporterV0>(Reporter::discriminator())?;

        reporters.retain(|(_, old_acc)| self.match_community(&old_acc.community));

        if reporters.is_empty() {
            println!("{}", "This program has no reporters to migrate\n".yellow());
        } else {
            println!("Starting migration of {} reporters", reporters.len());

            for (pk, reporter) in reporters {
                println!("Migrating reporter: {}", pk);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateReporter {
                        authority: self.cli.payer(),
                        community: reporter.community,
                        reporter: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateReporter)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All reporters migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_reporter_rewards(&self) -> Result<()> {
        println!(
            "{}",
            "Warning: reporter reward account can migrate only one time".yellow()
        );

        let mut reporter_rewards = vec![];

        {
            let rewards = self.get_program_accounts_with_discriminator::<ReporterRewardV0>(
                ReporterReward::discriminator(),
            )?;

            // Rewards could migrate only if reporter already migrated and it belongs to specified community
            for (pk, reward) in rewards {
                if let Ok(reporter) = self.cli.account::<Reporter>(reward.reporter) {
                    if self.match_community(&reporter.community) {
                        reporter_rewards.push((pk, reward, reporter));
                    }
                }
            }
        }

        if reporter_rewards.is_empty() {
            println!(
                "{}",
                "This program has no reporter rewards to migrate\n".yellow()
            );
        } else {
            println!(
                "Starting migration of {} reporter rewards",
                reporter_rewards.len()
            );

            for (pk, reward, reporter) in reporter_rewards {
                println!("Migrating reporter reward: {}", pk);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateReporterReward {
                        authority: self.cli.payer(),
                        community: reporter.community,
                        network: reward.network,
                        reporter: reward.reporter,
                        reporter_reward: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateReporterReward)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All reporter rewards migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_cases(&self) -> Result<()> {
        let mut cases =
            self.get_program_accounts_with_discriminator::<CaseV0>(Case::discriminator())?;

        cases.retain(|(_, old_acc)| self.match_community(&old_acc.community));

        if cases.is_empty() {
            println!("{}", "This program has no cases to migrate\n".yellow());
        } else {
            println!("Starting migration of {} cases", cases.len());

            for (pk, case) in cases {
                println!("Migrating case: {}", pk);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCase {
                        authority: self.cli.payer(),
                        community: case.community,
                        reporter: case.reporter,
                        case: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCase)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All cases migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_addresses(&self) -> Result<()> {
        let mut addresses =
            self.get_program_accounts_with_discriminator::<AddressV0>(Address::discriminator())?;

        addresses.retain(|(_, old_acc)| self.match_community(&old_acc.community));

        if addresses.is_empty() {
            println!("{}", "This program has no addresses to migrate\n".yellow());
        } else {
            println!("Starting migration of {} addresses", addresses.len());

            for (pk, address) in addresses {
                println!("Migrating reporter: {}", pk);

                let (case, _) = Pubkey::find_program_address(
                    &[
                        b"case".as_ref(),
                        address.community.as_ref(),
                        &address.case_id.to_le_bytes(),
                    ],
                    &self.cli.id(),
                );

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateAddress {
                        authority: self.cli.payer(),
                        community: address.community,
                        network: address.network,
                        reporter: address.reporter,
                        case,
                        address: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateAddress)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All addresses migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_assets(&self) -> Result<()> {
        let mut assets =
            self.get_program_accounts_with_discriminator::<AssetV0>(Asset::discriminator())?;

        assets.retain(|(_, old_acc)| self.match_community(&old_acc.community));

        if assets.is_empty() {
            println!("{}", "This program has no assets to migrate\n".yellow());
        } else {
            println!("Starting migration of {} assets", assets.len());

            for (pk, asset) in assets {
                println!("Migrating reporter: {}", pk);

                let (case, _) = Pubkey::find_program_address(
                    &[
                        b"case".as_ref(),
                        asset.community.as_ref(),
                        &asset.case_id.to_le_bytes(),
                    ],
                    &self.cli.id(),
                );

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateAsset {
                        authority: self.cli.payer(),
                        community: asset.community,
                        network: asset.network,
                        reporter: asset.reporter,
                        case,
                        asset: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateAsset)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All assets migrated\n".green());
        }

        Ok(())
    }
}

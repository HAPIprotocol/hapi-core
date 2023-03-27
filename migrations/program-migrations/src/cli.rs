use crate::configuration::{CommunityCfg, HapiCfg};

use {
    anchor_client::{
        anchor_lang::{AccountDeserialize, Discriminator},
        solana_sdk::{
            commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
        },
        Client, Cluster, Program,
    },
    anyhow::Result,
    colored::*,
    hapi_core::{
        accounts, id, instruction,
        state::{
            address::{Address, DeprecatedAddress},
            asset::{Asset, DeprecatedAsset},
            case::Case,
            community::{Community, DeprecatedCommunity},
            network::{DeprecatedNetwork, Network},
            reporter::{Reporter, ReporterReward},
        },
    },
    solana_program::{system_program, sysvar::rent},
    std::{rc::Rc, str::FromStr},
};

/// Returns rpc client
pub fn get_program(cfg: &HapiCfg) -> Program {
    let payer = read_keypair_file(cfg.keypair_path.clone()).expect("Failed to read keypair");
    let environment = Cluster::from_str(&cfg.environment).expect("Failed to initialize cluster");
    let program_id = if !cfg.program_id.is_empty() {
        cfg.program_id.parse::<Pubkey>().expect("Invalid pubkey")
    } else {
        id()
    };

    let client =
        Client::new_with_options(environment, Rc::new(payer), CommitmentConfig::processed());

    client.program(program_id)
}

pub struct HapiCli {
    cli: Program,
}

impl HapiCli {
    pub fn new(cfg: &HapiCfg) -> Self {
        Self {
            cli: get_program(cfg),
        }
    }

    fn get_program_accounts<T: AccountDeserialize + Discriminator>(
        &self,
    ) -> Result<Vec<(Pubkey, T)>> {
        Ok(self.cli.accounts::<T>(vec![])?)
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

    pub fn migrate_communities(&self, communities_cfg: &[CommunityCfg]) -> Result<()> {
        let communities = self.get_program_accounts_with_discriminator::<DeprecatedCommunity>(
            Community::discriminator(),
        )?;

        if communities.is_empty() {
            println!("{}", "This program has no communities\n".yellow());
        } else {
            println!("Starting migration of {} communities", communities.len());

            for (pk, community) in communities {
                let cfg = communities_cfg
                    .iter()
                    .find(|cfg| cfg.pubkey == pk.to_string())
                    .ok_or_else(|| {
                        anyhow::Error::msg(format!("Community {} is absent in config", pk))
                    })?;

                let treasury_token_account = Pubkey::from_str(&cfg.treasury_token_account)?;

                println!(
                    "Migrating community: {}, treasury token account: {}, appraiser stake: {}",
                    pk, cfg.treasury_token_account, cfg.appraiser_stake
                );

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCommunity {
                        authority: self.cli.payer(),
                        community: pk,
                        treasury_token_account,
                        stake_mint: community.stake_mint,
                        token_signer: community.token_signer,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCommunity {
                        appraiser_stake: cfg.appraiser_stake,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All communities migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_networks(&self) -> Result<()> {
        let networks = self.get_program_accounts_with_discriminator::<DeprecatedNetwork>(
            Network::discriminator(),
        )?;

        if networks.is_empty() {
            println!("{}", "This program has no networks\n".yellow());
        } else {
            println!("Starting migration of {} networks", networks.len());

            for (pk, network) in networks {
                println!("Migrating network: {}", pk);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateNetwork {
                        authority: self.cli.payer(),
                        community: network.community,
                        network: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateNetwork)
                    .send()?;

                println!("Migration success, signature {}", signature);
            }
            println!("{}", "All networks migrated\n".green());
        }

        Ok(())
    }

    pub fn migrate_reporters(&self) -> Result<()> {
        let reporters = self.get_program_accounts::<Reporter>()?;

        if reporters.is_empty() {
            println!("{}", "This program has no reporters\n".yellow());
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
        let rewards = self.get_program_accounts::<ReporterReward>()?;

        if rewards.is_empty() {
            println!("{}", "This program has no reporter rewards\n".yellow());
        } else {
            println!("Starting migration of {} reporter rewards", rewards.len());

            for (pk, reward) in rewards {
                println!("Migrating reporter reward: {}", pk);

                let reporter = self.cli.account::<Reporter>(reward.reporter)?;

                let (reporter_reward, _) = Pubkey::find_program_address(
                    &[
                        b"reporter_reward2".as_ref(),
                        reward.network.as_ref(),
                        reward.reporter.as_ref(),
                    ],
                    &self.cli.id(),
                );

                if pk == reporter_reward {
                    println!("{}", "Account already migrated".yellow());
                    continue;
                }

                println!("New reporter reward account: {}", reporter_reward);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateReporterReward {
                        authority: self.cli.payer(),
                        community: reporter.community,
                        network: reward.network,
                        reporter: reward.reporter,
                        reporter_reward,
                        deprecated_reporter_reward: pk,
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
        let cases = self.get_program_accounts::<Case>()?;

        if cases.is_empty() {
            println!("{}", "This program has no cases\n".yellow());
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
        let addresses = self.get_program_accounts_with_discriminator::<DeprecatedAddress>(
            Address::discriminator(),
        )?;

        if addresses.is_empty() {
            println!("{}", "This program has no addresses\n".yellow());
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
        let assets = self
            .get_program_accounts_with_discriminator::<DeprecatedAsset>(Asset::discriminator())?;

        if assets.is_empty() {
            println!("{}", "This program has no assets\n".yellow());
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

use crate::configuration::HapiCfg;

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
    communities: Vec<String>,
}

impl HapiCli {
    pub fn new(cfg: &HapiCfg) -> Result<Self> {
        Ok(Self {
            cli: get_program(cfg)?,
            communities: cfg.communities.clone(),
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

    fn get_community(&self, id: usize) -> (Pubkey, u8) {
        let seeds: [&[u8]; 2] = [b"community".as_ref(), &id.to_le_bytes()];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_network(&self, community: &Pubkey, name: [u8; 32]) -> (Pubkey, u8) {
        let seeds: [&[u8]; 3] = [b"network".as_ref(), community.as_ref(), &name];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    pub fn migrate_communities(&self) -> Result<()> {
        let communities = self
            .get_program_accounts_with_discriminator::<CommunityV0>(Community::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in communities {
            if let Ok(community_id) = self.communities.binary_search(&pk.to_string()) {
                let (community, bump) = self.get_community(community_id);
                println!(
                    "Migrating community: {}, new community pda: {}",
                    pk, community
                );

                self.cli
                    .request()
                    .instruction(create_associated_token_account(
                        &self.cli.payer(),
                        &community,
                        &data.stake_mint,
                        &spl_token::ID,
                    ))
                    .send()?;

                let token_account = get_associated_token_address(&community, &data.stake_mint);
                println!("New community ATA: {}", token_account);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCommunity {
                        authority: self.cli.payer(),
                        old_community: pk,
                        community,
                        stake_mint: data.stake_mint,
                        token_signer: data.token_signer,
                        old_token_account: data.token_account,
                        token_account,
                        token_program: spl_token::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCommunity {
                        token_signer_bump: data.token_signer_bump,
                        community_id: community_id as u64,
                        bump,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " communities migrated\n".green());
        Ok(())
    }

    pub fn migrate_networks(&self) -> Result<()> {
        let networks =
            self.get_program_accounts_with_discriminator::<NetworkV0>(Network::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in networks {
            if let Ok(community_id) = self.communities.binary_search(&data.community.to_string()) {
                let (community, _) = self.get_community(community_id);
                let (network, bump) = self.get_network(&community, data.name);
                println!("Migrating network: {}", pk);

                self.cli
                    .request()
                    .instruction(create_associated_token_account(
                        &self.cli.payer(),
                        &network,
                        &data.reward_mint,
                        &spl_token::ID,
                    ))
                    .send()?;

                let treasury_token_account = get_associated_token_address(&pk, &data.reward_mint);

                println!("Network treasury ATA: {}", treasury_token_account);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateNetwork {
                        authority: self.cli.payer(),
                        community,
                        old_network: pk,
                        network,
                        reward_signer: data.reward_signer,
                        reward_mint: data.reward_mint,
                        treasury_token_account,
                        token_program: spl_token::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateNetwork {
                        bump,
                        name: data.name,
                        reward_signer_bump: data.reward_signer_bump,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " networks migrated\n".green());
        Ok(())
    }

    pub fn migrate_reporters(&self) -> Result<()> {
        let reporters =
            self.get_program_accounts_with_discriminator::<ReporterV0>(Reporter::discriminator())?;
        let mut migrations = 0;

        for (pk, reporter) in reporters {
            if let Ok(community_id) = self
                .communities
                .binary_search(&reporter.community.to_string())
            {
                println!("Migrating reporter: {}", pk);
                let (community, _) = self.get_community(community_id);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateReporter {
                        authority: self.cli.payer(),
                        community,
                        reporter: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateReporter)
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " reporters migrated\n".green());
        Ok(())
    }

    pub fn migrate_reporter_rewards(&self) -> Result<()> {
        let reporter_rewards = self.get_program_accounts_with_discriminator::<ReporterRewardV0>(
            ReporterReward::discriminator(),
        )?;
        let mut migrations = 0;

        for (pk, reward) in reporter_rewards {
            if let Ok(reporter) = self.cli.account::<Reporter>(reward.reporter) {
                if let Ok(community_id) = self
                    .communities
                    .binary_search(&reporter.community.to_string())
                {
                    println!("Migrating reporter reward: {}", pk);
                    let (community, _) = self.get_community(community_id);

                    let signature = self
                        .cli
                        .request()
                        .accounts(accounts::MigrateReporterReward {
                            authority: self.cli.payer(),
                            community,
                            network: reward.network,
                            reporter: reward.reporter,
                            reporter_reward: pk,
                            rent: rent::ID,
                            system_program: system_program::ID,
                        })
                        .args(instruction::MigrateReporterReward)
                        .send()?;

                    println!("Migration success, signature {}", signature);
                    migrations += 1;
                }
            }
        }

        println!("{}{}", migrations, " reporter rewards migrated\n".green());
        Ok(())
    }

    pub fn migrate_cases(&self) -> Result<()> {
        let cases = self.get_program_accounts_with_discriminator::<CaseV0>(Case::discriminator())?;
        let mut migrations = 0;

        for (pk, case) in cases {
            if let Ok(community_id) = self.communities.binary_search(&case.community.to_string()) {
                println!("Migrating case: {}", pk);
                let (community, _) = self.get_community(community_id);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCase {
                        authority: self.cli.payer(),
                        community,
                        reporter: case.reporter,
                        case: pk,
                        rent: rent::ID,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCase)
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " cases migrated\n".green());
        Ok(())
    }

    pub fn migrate_addresses(&self) -> Result<()> {
        let addresses =
            self.get_program_accounts_with_discriminator::<AddressV0>(Address::discriminator())?;
        let mut migrations = 0;

        for (pk, address) in addresses {
            if let Ok(community_id) = self
                .communities
                .binary_search(&address.community.to_string())
            {
                println!("Migrating address: {}", pk);
                let (community, _) = self.get_community(community_id);

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
                        community,
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
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " addresses migrated\n".green());
        Ok(())
    }

    pub fn migrate_assets(&self) -> Result<()> {
        let assets =
            self.get_program_accounts_with_discriminator::<AssetV0>(Asset::discriminator())?;

        let mut migrations = 0;

        for (pk, asset) in assets {
            if let Ok(community_id) = self.communities.binary_search(&asset.community.to_string()) {
                println!("Migrating address: {}", pk);
                let (community, _) = self.get_community(community_id);

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
                        community,
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
                migrations += 1;
            }
        }

        println!("{}{}", migrations, " assets migrated\n".green());
        Ok(())
    }
}

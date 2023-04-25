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
        get_associated_token_address, instruction::create_associated_token_account,
        solana_program::system_program,
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
    communities: Vec<CommunityCfg>,
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

    fn get_community_id(&self, pk: &Pubkey) -> Option<u64> {
        let pk = pk.to_string();
        self.communities
            .iter()
            .find(|cfg| cfg.pubkey == pk)
            .map(|cfg| cfg.id)
    }

    fn get_community(&self, id: u64) -> (Pubkey, u8) {
        let seeds: [&[u8]; 2] = [b"community".as_ref(), &id.to_le_bytes()];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_network(&self, community: &Pubkey, name: [u8; 32]) -> (Pubkey, u8) {
        let seeds: [&[u8]; 3] = [b"network".as_ref(), community.as_ref(), &name];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_reporter(&self, community: &Pubkey, pubkey: &Pubkey) -> (Pubkey, u8) {
        let seeds: [&[u8]; 3] = [b"reporter".as_ref(), community.as_ref(), pubkey.as_ref()];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_reporter_reward(&self, network: &Pubkey, reporter: &Pubkey) -> (Pubkey, u8) {
        let seeds: [&[u8]; 3] = [
            b"reporter_reward".as_ref(),
            network.as_ref(),
            reporter.as_ref(),
        ];

        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_case(&self, community: &Pubkey, case_id: u64) -> (Pubkey, u8) {
        let seeds: [&[u8]; 3] = [b"case".as_ref(), community.as_ref(), &case_id.to_le_bytes()];
        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_address(&self, network: &Pubkey, addr: [u8; 64]) -> (Pubkey, u8) {
        let seeds: [&[u8]; 4] = [
            b"address".as_ref(),
            network.as_ref(),
            addr[0..32].as_ref(),
            addr[32..64].as_ref(),
        ];

        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    fn get_asset(&self, network: &Pubkey, mint: [u8; 64], asset_id: [u8; 32]) -> (Pubkey, u8) {
        let seeds: [&[u8]; 5] = [
            b"asset".as_ref(),
            network.as_ref(),
            mint[0..32].as_ref(),
            mint[32..64].as_ref(),
            asset_id.as_ref(),
        ];

        Pubkey::find_program_address(&seeds, &self.cli.id())
    }

    pub fn migrate_communities(&self) -> Result<()> {
        let communities = self
            .get_program_accounts_with_discriminator::<CommunityV0>(Community::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in communities {
            if let Some(community_id) = self.get_community_id(&pk) {
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

        println!(
            "{}",
            format!("All communities migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_networks(&self) -> Result<()> {
        let networks =
            self.get_program_accounts_with_discriminator::<NetworkV0>(Network::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in networks {
            if let Some(community_id) = self.get_community_id(&data.community) {
                let (community, _) = self.get_community(community_id);
                let (network, bump) = self.get_network(&community, data.name);
                println!("Migrating network: {}, new network pda: {}", pk, network);

                self.cli
                    .request()
                    .instruction(create_associated_token_account(
                        &self.cli.payer(),
                        &network,
                        &data.reward_mint,
                        &spl_token::ID,
                    ))
                    .send()?;

                let treasury_token_account =
                    get_associated_token_address(&network, &data.reward_mint);

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

        println!(
            "{}",
            format!("All networks migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_reporters(&self) -> Result<()> {
        let reporters =
            self.get_program_accounts_with_discriminator::<ReporterV0>(Reporter::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in reporters {
            if let Some(community_id) = self.get_community_id(&data.community) {
                let (community, _) = self.get_community(community_id);
                let (reporter, bump) = self.get_reporter(&community, &data.pubkey);
                println!("Migrating reporter: {}, new reporter pda: {}", pk, reporter);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateReporter {
                        authority: self.cli.payer(),
                        community,
                        old_reporter: pk,
                        pubkey: data.pubkey,
                        reporter,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateReporter { bump })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!(
            "{}",
            format!("All reporters migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_reporter_rewards(&self) -> Result<()> {
        let reporter_rewards = self.get_program_accounts_with_discriminator::<ReporterRewardV0>(
            ReporterReward::discriminator(),
        )?;
        let mut migrations = 0;

        for (pk, data) in reporter_rewards {
            if let Ok(reporter) = self.cli.account::<Reporter>(data.reporter) {
                if let Some(community_id) = self.get_community_id(&reporter.community) {
                    let (community, _) = self.get_community(community_id);
                    let (reporter_reward, bump) =
                        self.get_reporter_reward(&data.network, &data.reporter);
                    println!(
                        "Migrating reporter reward: {}, new reporter reward pda: {}",
                        pk, reporter_reward
                    );

                    let signature = self
                        .cli
                        .request()
                        .accounts(accounts::MigrateReporterReward {
                            authority: self.cli.payer(),
                            community,
                            network: data.network,
                            reporter: data.reporter,
                            old_reporter_reward: pk,
                            reporter_reward,
                            system_program: system_program::ID,
                        })
                        .args(instruction::MigrateReporterReward { bump })
                        .send()?;

                    println!("Migration success, signature {}", signature);
                    migrations += 1;
                }
            }
        }

        println!(
            "{}",
            format!("All reporter rewards migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_cases(&self) -> Result<()> {
        let cases = self.get_program_accounts_with_discriminator::<CaseV0>(Case::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in cases {
            if let Some(community_id) = self.get_community_id(&data.community) {
                let (community, _) = self.get_community(community_id);
                let (case, bump) = self.get_case(&community, data.id);
                println!("Migrating case: {}, new case pda: {}", pk, case);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateCase {
                        authority: self.cli.payer(),
                        community,
                        old_case: pk,
                        case,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateCase {
                        bump,
                        case_id: data.id,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!(
            "{}",
            format!("All cases migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_addresses(&self) -> Result<()> {
        let addresses =
            self.get_program_accounts_with_discriminator::<AddressV0>(Address::discriminator())?;
        let mut migrations = 0;

        for (pk, data) in addresses {
            if let Some(community_id) = self.get_community_id(&data.community) {
                let (community, _) = self.get_community(community_id);
                let (address, bump) = self.get_address(&data.network, data.address);
                println!("Migrating address: {}, new address pda: {}", pk, address);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateAddress {
                        authority: self.cli.payer(),
                        community,
                        network: data.network,
                        old_address: pk,
                        address,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateAddress {
                        bump,
                        addr: data.address,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!(
            "{}",
            format!("All addresses migrated: {} migartions\n", migrations).green()
        );
        Ok(())
    }

    pub fn migrate_assets(&self) -> Result<()> {
        let assets =
            self.get_program_accounts_with_discriminator::<AssetV0>(Asset::discriminator())?;

        let mut migrations = 0;

        for (pk, data) in assets {
            if let Some(community_id) = self.get_community_id(&data.community) {
                let (community, _) = self.get_community(community_id);
                let (asset, bump) = self.get_asset(&data.network, data.mint, data.asset_id);
                println!("Migrating asset: {}, new asset pda: {}", pk, asset);

                let signature = self
                    .cli
                    .request()
                    .accounts(accounts::MigrateAsset {
                        authority: self.cli.payer(),
                        community,
                        network: data.network,
                        old_asset: pk,
                        asset,
                        system_program: system_program::ID,
                    })
                    .args(instruction::MigrateAsset {
                        bump,
                        mint: data.mint,
                        asset_id: data.asset_id,
                    })
                    .send()?;

                println!("Migration success, signature {}", signature);
                migrations += 1;
            }
        }

        println!(
            "{}",
            format!("All assets migrated: {} migrations\n", migrations).green()
        );
        Ok(())
    }
}

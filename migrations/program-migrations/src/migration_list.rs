use {
    anyhow::{bail, Result},
    colored::*,
    serde_derive::{Deserialize, Serialize},
    serde_with::serde_as,
    std::fs,
    {anchor_client::solana_sdk::pubkey::Pubkey, std::collections::HashMap},
};

use crate::configuration::MigrateAccount;

const INPUT_PATH: &str = "";

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationList {
    #[serde_as(as = "Vec<(_, _)>")]
    pub communities: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub networks: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub reporters: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub cases: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub rewards: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub addresses: HashMap<Pubkey, Pubkey>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub assets: HashMap<Pubkey, Pubkey>,
}

impl MigrationList {
    pub fn new() -> Result<Self> {
        Ok(serde_json::from_str::<MigrationList>(&fs::read_to_string(
            INPUT_PATH,
        )?)?)
    }

    fn get_list(&mut self, acc: MigrateAccount) -> &mut HashMap<Pubkey, Pubkey> {
        match acc {
            MigrateAccount::Community => &mut self.communities,
            MigrateAccount::Network => &mut self.networks,
            MigrateAccount::Reporter => &mut self.reporters,
            MigrateAccount::ReporterReward => &mut self.rewards,
            MigrateAccount::Case => &mut self.cases,
            MigrateAccount::Address => &mut self.addresses,
            MigrateAccount::Asset => &mut self.assets,
            _ => {
                panic!("Invalid account")
            }
        }
    }

    pub fn add_account(&mut self, acc: MigrateAccount, old: Pubkey, new: Pubkey) -> Result<()> {
        let list = self.get_list(acc);
        list.insert(old, new);

        Ok(fs::write(INPUT_PATH, serde_json::to_string(&self)?)?)
    }

    pub fn get_account(&mut self, acc: MigrateAccount, pk: &Pubkey) -> Option<&Pubkey> {
        let list = self.get_list(acc);
        list.get(pk)
    }

    pub fn print_migrations(&self) {
        println!("{}", "Migrated communities:".yellow());
        print_accs(&self.communities);
        println!("{}", "Migrated networks:".yellow());
        print_accs(&self.networks);
        println!("{}", "Migrated reporters:".yellow());
        print_accs(&self.reporters);
        println!("{}", "Migrated rewards:".yellow());
        print_accs(&self.rewards);
        println!("{}", "Migrated cases:".yellow());
        print_accs(&self.cases);
        println!("{}", "Migrated addresses:".yellow());
        print_accs(&self.addresses);
        println!("{}", "Migrated assets:".yellow());
        print_accs(&self.assets);
    }
}

fn print_accs(accs: &HashMap<Pubkey, Pubkey>) {
    for (old, new) in accs {
        println!("{} -> {}", old, new);
    }
    println!("");
}

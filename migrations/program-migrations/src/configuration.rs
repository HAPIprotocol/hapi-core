use {
    config::{Config, ConfigError, File},
    serde_derive::Deserialize,
    std::env,
};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CommunityCfg {
    pub pubkey: String,
    pub treasury_token_account: String,
    pub appraiser_stake: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub enum MigrateAccount {
    All,
    Community,
    Network,
    Reporter,
    ReporterReward,
    Case,
    Address,
    Asset,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct HapiCfg {
    pub keypair_path: String,
    #[serde(default)]
    pub program_id: String,
    #[serde(default = "localhost_node")]
    pub environment: String,
    pub communities: Vec<CommunityCfg>,
    #[serde(default = "migrate_all")]
    pub migrate_accounts: Vec<MigrateAccount>,
}

impl HapiCfg {
    pub fn build() -> Result<Self, ConfigError> {
        let config = env::var("HAPI_CFG").unwrap_or_else(|_| "./config.yaml".into());

        let s = Config::builder()
            .add_source(File::with_name(&config).required(true))
            .build()?;

        s.try_deserialize().map_err(Into::into)
    }
}

fn localhost_node() -> String {
    "localnet".into()
}

fn migrate_all() -> Vec<MigrateAccount> {
    vec![MigrateAccount::All]
}

use {
    config::{Config, ConfigError, File, FileFormat},
    hapi_core::HapiCoreNetwork,
    serde::Deserialize,
    serde_with::{serde_as, DurationMilliSeconds},
    std::{env, time::Duration},
};

pub const CONFIG_PATH: &str = "configuration.toml";

#[serde_as]
#[derive(Deserialize, Clone)]
pub struct Configuration {
    /// Log level for the application layer
    #[serde(default = "default_loglevel")]
    pub log_level: String,

    /// Whether to use JSON logging
    #[serde(default = "default_is_json_logging")]
    pub is_json_logging: bool,

    /// The address to listen on
    #[serde(default = "default_listener")]
    pub listener: String,

    pub indexer: IndexerConfiguration,
}

#[serde_as]
#[derive(Deserialize, Clone)]
pub struct IndexerConfiguration {
    /// The network to use
    pub network: HapiCoreNetwork,

    /// Optionsl network chain ID
    pub chain_id: Option<String>,

    /// The RPC node URL
    pub rpc_node_url: String,

    /// URL to send webhooks to
    pub webhook_url: String,

    /// The HAPI Core contract address
    pub contract_address: String,

    /// The number of milliseconds between wait checks
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    #[serde(default = "default_wait_tick")]
    pub wait_interval_ms: Duration,

    /// The file to persist the indexer state in
    #[serde(default = "default_state_file")]
    pub state_file: String,

    /// The number of milliseconds between iterations in the fetching
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    #[serde(default = "default_delay")]
    pub fetching_delay: Duration,

    /// JWT token to use for the webhook
    pub jwt_token: String,
}

fn default_is_json_logging() -> bool {
    true
}

fn default_loglevel() -> String {
    String::from("info")
}

fn default_listener() -> String {
    String::from("0.0.0.0:3000")
}

fn default_wait_tick() -> Duration {
    Duration::from_millis(1000)
}

fn default_delay() -> Duration {
    Duration::from_millis(100)
}

fn default_state_file() -> String {
    String::from("data/state.json")
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| CONFIG_PATH.to_string());

    let settings = Config::builder()
        .add_source(
            File::with_name(&config_path)
                .format(FileFormat::Toml)
                .required(true),
        )
        .build()?;

    settings.try_deserialize::<Configuration>()
}

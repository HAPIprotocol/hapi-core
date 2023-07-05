use {
    config::{Config, ConfigError, File, FileFormat},
    serde::Deserialize,
    serde_with::{serde_as, DurationMilliSeconds},
    std::{env, time::Duration},
};

use crate::indexer::Network;

pub const CONFIG_PATH: &str = "configuration.toml";
pub const SECRET_PATH: &str = "secret.toml";

#[serde_as]
#[derive(Default, Deserialize, Clone)]
pub(crate) struct Configuration {
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
#[derive(Default, Deserialize, Clone)]
pub(crate) struct IndexerConfiguration {
    /// The network to use
    #[serde(deserialize_with = "Network::deserialize")]
    pub network: Network,

    /// The RPC node URL
    pub rpc_node_url: String,

    /// The HAPI Core contract address
    pub contract_address: String,

    /// The number of milliseconds between wait checks
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    #[serde(default = "default_wait_tick")]
    pub wait_interval_ms: Duration,
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

pub(crate) fn get_configuration() -> Result<Configuration, ConfigError> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| CONFIG_PATH.to_string());
    let secret_path = env::var("SECRET_PATH").unwrap_or_else(|_| SECRET_PATH.to_string());

    let settings = Config::builder()
        .add_source(
            File::with_name(&config_path)
                .format(FileFormat::Toml)
                .required(true),
        )
        .add_source(
            File::with_name(&secret_path)
                .format(FileFormat::Toml)
                .required(false),
        )
        .build()?;

    settings.try_deserialize::<Configuration>()
}

use {
    config::{Config, ConfigError, File, FileFormat},
    serde::Deserialize,
    serde_with::serde_as,
    std::env,
};

const CONFIG_PATH: &str = "configuration.toml";

#[serde_as]
#[derive(Default, Deserialize, Clone)]
pub struct Configuration {
    /// Log level for the application layer
    #[serde(default = "default_loglevel")]
    pub log_level: String,

    /// Whether to use JSON logging
    #[serde(default = "default_is_json_logging")]
    pub is_json_logging: bool,

    /// Whether to enable metrics
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,

    /// The address to listen on
    #[serde(default = "default_listener")]
    pub listener: String,

    // TODO: move to separate struct
    /// The database url
    pub database_url: String,
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

fn default_loglevel() -> String {
    String::from("info")
}

fn default_is_json_logging() -> bool {
    true
}

fn default_listener() -> String {
    String::from("0.0.0.0:3000")
}

fn default_enable_metrics() -> bool {
    true
}

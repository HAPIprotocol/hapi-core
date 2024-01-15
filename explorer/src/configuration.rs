use {
    config::{Config, ConfigError, File, FileFormat},
    secrecy::SecretString,
    serde::{Deserialize, Deserializer},
    serde_with::serde_as,
    std::env,
};

const CONFIG_PATH: &str = "configuration.toml";
const SECRET_PATH: &str = "secret.toml";

#[serde_as]
#[derive(Deserialize, Clone)]
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

    /// The database url
    pub database_url: String,

    /// Secret for JWT
    #[serde(deserialize_with = "deserialize_secret_string")]
    pub jwt_secret: SecretString,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            log_level: default_loglevel(),
            is_json_logging: default_is_json_logging(),
            enable_metrics: default_enable_metrics(),
            listener: default_listener(),
            database_url: String::new(),
            jwt_secret: default_jwt_secret(),
        }
    }
}

pub fn get_configuration() -> Result<Configuration, ConfigError> {
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

fn default_jwt_secret() -> SecretString {
    SecretString::new("my_ultra_secure_secret".to_string())
}

fn deserialize_secret_string<'de, D>(deserializer: D) -> Result<SecretString, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SecretString::new(s))
}

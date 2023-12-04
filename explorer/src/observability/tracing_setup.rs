use {
    anyhow::{anyhow, Result},
    tracing::subscriber,
    tracing_subscriber::{fmt::Subscriber, EnvFilter},
};

pub fn setup_tracing(log_level: &str, is_json_logging: bool) -> Result<()> {
    let builder = Subscriber::builder()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}={log_level}", to_snake_case(env!("CARGO_PKG_NAME"))).into()
        }))
        .with_writer(std::io::stdout);

    let result = if is_json_logging {
        subscriber::set_global_default(builder.json().flatten_event(true).finish())
    } else {
        subscriber::set_global_default(builder.finish())
    };

    result.map_err(|e| anyhow!("Failed to set up tracing subscriber: {:?}", e))
}

fn to_snake_case(s: &str) -> String {
    s.to_lowercase().replace(['-', ' '], "_")
}

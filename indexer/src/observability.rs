use {
    tracing::subscriber,
    tracing_subscriber::{fmt::Subscriber, EnvFilter},
};

pub(crate) fn setup_tracing(log_level: &str) {
    let subscriber = Subscriber::builder()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}={log_level}", to_snake_case(env!("CARGO_PKG_NAME"))).into()
        }))
        .with_writer(std::io::stdout)
        .finish();

    subscriber::set_global_default(subscriber).expect("Failed to set up tracing subscriber");
}

pub(crate) fn setup_json_tracing(log_level: &str) {
    let subscriber = Subscriber::builder()
        .json()
        .flatten_event(true)
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}={log_level}", to_snake_case(env!("CARGO_PKG_NAME"))).into()
        }))
        .with_writer(std::io::stdout)
        .finish();

    subscriber::set_global_default(subscriber).expect("Failed to set up tracing subscriber");
}

fn to_snake_case(s: &str) -> String {
    s.to_lowercase().replace(['-', ' '], "_")
}

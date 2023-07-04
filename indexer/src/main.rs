use {
    anyhow::{Error, Result},
    std::env::var,
    tokio::{task::spawn, try_join},
    tracing_subscriber::fmt::Subscriber,
};

mod indexer;
use indexer::{Indexer, Network};

fn setup_observability() {
    let subscriber = Subscriber::builder()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hapi_indexer=trace".into()),
        )
        .with_writer(std::io::stdout)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up tracing subscriber");
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_observability();

    let network: Network = var("NETWORK")
        .unwrap_or_else(|_| "ethereum".to_string())
        .parse()?;

    let mut indexer = Indexer::new(network);

    let server_task = indexer.spawn_server("0.0.0.0:3000").await?;

    tracing::debug!(port = 3000, "Start server");

    let indexer_task = spawn(async move {
        indexer.run().await.or_else(|error: Error| -> Result<()> {
            tracing::error!(?error, "Indexer failed");
            Ok(())
        })
    });

    let _ = try_join!(server_task, indexer_task)?;

    Ok(())
}

use {
    anyhow::{Error, Result},
    tokio::{task::spawn, try_join},
};

mod config;
mod indexer;
mod observability;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::get_configuration()
        .map_err(|e| anyhow::anyhow!("Configuration parsing error: {e}"))?;

    if cfg.is_json_logging {
        observability::setup_json_tracing(&cfg.log_level);
    } else {
        observability::setup_tracing(&cfg.log_level);
    }

    tracing::info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let mut indexer = indexer::Indexer::new(cfg.indexer)?;

    let server_task = indexer.spawn_server(&cfg.listener).await?;

    let indexer_task = spawn(async move {
        indexer.run().await.or_else(|error: Error| -> Result<()> {
            tracing::error!(?error, "Indexer failed");
            Ok(())
        })
    });

    let _ = try_join!(server_task, indexer_task)?;

    Ok(())
}

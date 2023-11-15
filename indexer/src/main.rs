use {
    anyhow::{bail, Result},
    tokio::{task::spawn, try_join},
};

use hapi_indexer::{
    configuration::get_configuration,
    observability::{setup_json_tracing, setup_tracing},
    Indexer,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg =
        get_configuration().map_err(|e| anyhow::anyhow!("Configuration parsing error: {e}"))?;

    if cfg.is_json_logging {
        setup_json_tracing(&cfg.log_level)?;
    } else {
        setup_tracing(&cfg.log_level)?;
    }

    tracing::info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let mut indexer = Indexer::new(cfg.indexer)?;

    let server_task = indexer.spawn_server(&cfg.listener).await?;
    let indexer_task = spawn(async move { indexer.run().await });

    match try_join!(server_task, indexer_task)? {
        (Err(e), _) | (_, Err(e)) => {
            tracing::error!(?e, "Indexer failed");

            bail!("Indexer failed with error: {:?}", e);
        }
        _ => Ok(()),
    }
}

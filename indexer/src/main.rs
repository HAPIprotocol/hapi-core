use {
    anyhow::{bail, Result},
    tokio::{
        select,
        task::{spawn, JoinError},
    },
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

    select! {
        server_result = server_task => {
            handle_result(server_result).await
        }
        indexer_result = indexer_task => {
            handle_result(indexer_result).await
        }
    }
}

async fn handle_result(result: Result<Result<(), anyhow::Error>, JoinError>) -> Result<()> {
    match result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => {
            tracing::error!(?e, "Indexer failed");
            bail!("Indexer failed with error: {:?}", e);
        }
        Err(e) => {
            tracing::error!(?e, "Task failed to execute to completion");
            bail!("Task failed to execute to completion: {:?}", e);
        }
    }
}

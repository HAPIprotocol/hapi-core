use {
    anyhow::Result,
    axum::{middleware, routing::get, Router, Server},
    std::future::ready,
    tokio::task::{spawn, JoinHandle},
};

use super::{entities, health, stats};
use crate::{
    application::Application,
    observability::{setup_metrics, track_metrics},
};

impl Application {
    async fn shutdown_signal() {
        unimplemented!()
    }

    fn create_router(&self) -> Router {
        let router = Router::new()
            .route("/health", get(health))
            .route("/entities", get(entities))
            .route("/stats", get(stats));

        if self.enable_metrics {
            let prometheus_recorder = setup_metrics();

            // TODO: allow access only to the admin
            return router
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics));
        }

        router
    }

    pub async fn spawn_server(&self) -> Result<JoinHandle<Result<()>>> {
        tracing::info!(address = ?self.socket, "Start server");

        let server = Server::bind(&self.socket).serve(self.create_router().into_make_service());
        // .with_graceful_shutdown(shutdown_signal());

        Ok(spawn(
            async move { server.await.map_err(anyhow::Error::from) },
        ))
    }
}

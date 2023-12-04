use {
    anyhow::Result,
    axum::{
        middleware,
        routing::{get, post},
        serve, Router,
    },
    std::future::ready,
    tokio::net::TcpListener,
};

use super::{events, health, stats};
use crate::{
    application::Application,
    observability::{setup_metrics, track_metrics},
};

impl Application {
    fn create_router(&self) -> Router {
        let router = Router::new()
            .route("/health", get(health))
            .route("/events", post(events))
            .route("/stats", get(stats))
            .with_state(self.database_conn.clone());

        if self.enable_metrics {
            let prometheus_recorder = setup_metrics();

            // TODO: allow access only to the admin
            return router
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics));
        }

        router
    }

    pub async fn run_server(self) -> Result<()> {
        tracing::info!(address = ?self.socket, "Start server");

        // TODO: implement graceful shutdown
        let server: serve::Serve<axum::routing::IntoMakeService<Router>, Router> = serve(
            TcpListener::bind(self.socket).await?,
            self.create_router().into_make_service(),
        );

        server.await.map_err(anyhow::Error::from)
    }
}

use {
    anyhow::Result,
    axum::{
        middleware,
        routing::{get, post},
        serve, Router,
    },
    std::future::ready,
    tokio::{
        net::TcpListener,
        task::{spawn, JoinHandle},
    },
};

use super::{events, health, stats};
use crate::{
    application::Application,
    observability::{setup_metrics, track_metrics},
};

impl Application {
    async fn shutdown_signal(self) {
        self.database_conn
            .close()
            .await
            .expect("Failed to close database connection");
    }

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

    pub async fn spawn_server(self) -> Result<JoinHandle<Result<()>>> {
        tracing::info!(address = ?self.socket, "Start server");

        // let server = Server::bind(&self.socket).serve(self.create_router().into_make_service());
        // // TODO: fix graceful shutdown
        // // .with_graceful_shutdown(self.shutdown_signal());

        let server = serve(
            TcpListener::bind(self.socket).await?,
            self.create_router().into_make_service(),
        );

        Ok(spawn(
            async move { server.await.map_err(anyhow::Error::from) },
        ))
    }
}

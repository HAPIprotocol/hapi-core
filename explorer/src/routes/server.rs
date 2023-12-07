use {
    anyhow::Result,
    axum::{
        middleware,
        routing::{get, post},
        serve, Router,
    },
    secrecy::SecretString,
    std::future::ready,
    std::sync::Arc,
    tokio::net::TcpListener,
};

use super::{auth, events, health, stats};
use crate::{
    application::Application,
    observability::{setup_metrics, track_metrics},
};

#[derive(Clone)]
pub struct AppState {
    pub database_conn: sea_orm::DatabaseConnection,
    pub jwt_secret: Arc<SecretString>,
}

impl Application {
    fn create_router(&self, jwt_secret: Arc<SecretString>) -> Router {
        let app_state = AppState {
            database_conn: self.database_conn.clone(),
            jwt_secret,
        };
        let router = Router::new()
            .route("/health", get(health))
            .route(
                "/events",
                post(events).route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
            )
            .route("/stats", get(stats))
            .with_state(app_state);

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

        let jwt_token = Arc::new(self.jwt_secret.clone());
        // TODO: implement graceful shutdown
        let server: serve::Serve<axum::routing::IntoMakeService<Router>, Router> = serve(
            TcpListener::bind(self.socket).await?,
            self.create_router(jwt_token).into_make_service(),
        );

        server.await.map_err(anyhow::Error::from)
    }
}

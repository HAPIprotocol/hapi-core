use {
    anyhow::{anyhow, Result},
    axum::{
        middleware,
        routing::{get, post, put},
        Extension, Router, Server,
    },
    std::future::ready,
    tokio::{signal, sync::oneshot},
    tracing::info,
};

use super::{
    handlers::{
        auth_handler, event_handler, graphiql_playground, graphql_handler, health_handler,
        indexer_handler, indexer_heartbeat_handler, stats_handler,
    },
    schema::create_graphql_schema,
};

use crate::{
    application::Application,
    observability::{setup_metrics, track_metrics},
};

impl Application {
    fn create_router(&self) -> Result<Router> {
        let schema = create_graphql_schema(self.state.database_conn.clone())?;

        let router = Router::new()
            .route("/health", get(health_handler))
            .route(
                "/events",
                post(event_handler).route_layer(middleware::from_fn_with_state(
                    self.state.clone(),
                    auth_handler,
                )),
            )
            .route("/stats", get(stats_handler))
            .route("/graphql", get(graphiql_playground).post(graphql_handler))
            .route("/indexer", get(indexer_handler))
            .route("/indexer/:id/heartbeat", put(indexer_heartbeat_handler))
            .with_state(self.state.clone())
            .layer(Extension(schema));

        if self.enable_metrics {
            let prometheus_recorder = setup_metrics();

            // TODO: allow access only to the admin
            return Ok(router
                .route("/metrics", get(move || ready(prometheus_recorder.render())))
                .route_layer(middleware::from_fn(track_metrics)));
        }

        Ok(router)
    }

    pub async fn run_server(&mut self) -> Result<()> {
        tracing::info!(address = ?self.socket, "Start server");

        let (tx, rx) = oneshot::channel::<()>();
        self.shutdown_sender = Some(tx);

        let server = Server::bind(&self.socket)
            .serve(self.create_router()?.into_make_service())
            .with_graceful_shutdown(async {
                rx.await.ok();
                info!("Signal received, starting graceful shutdown");
            });

        // Store the server task's handle
        self.server_handle = Some(tokio::spawn(
            async move { server.await.map_err(|e| anyhow!(e)) },
        ));

        Ok(())
    }

    pub async fn handle_shutdown_signal(&mut self) -> Result<()> {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        let interrupt = async {
            signal::unix::signal(signal::unix::SignalKind::interrupt())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        let quit = async {
            signal::unix::signal(signal::unix::SignalKind::quit())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        tokio::select! {
                _ = ctrl_c => info!("Ctrl-c received!"),
                _ = terminate => info!("Terminate received!"),
                _ = interrupt => info!("Interrupt received!"),
                _ = quit => info!("Quit received!"),
        };

        self.shutdown().await
    }
}

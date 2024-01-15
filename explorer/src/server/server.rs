use {
    anyhow::Result,
    axum::{
        middleware,
        routing::{get, post, put},
        Extension, Router, Server,
    },
    std::future::ready,
};

use super::{
    handlers::{
        auth_handler, event_handler, graphiql, graphql_handler, health_handler, indexer_handler,
        indexer_heartbeat_handler, stats_handler,
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
            .route("/graphql", get(graphiql).post(graphql_handler))
            .route("/indexer", get(indexer_handler))
            .route("/indexer/:id/heartbeat", put(indexer_heartbeat_handler))
            .with_state(self.state.clone())
            .layer(Extension(schema));

        // if self.enable_metrics {
        //     let prometheus_recorder = setup_metrics();

        //     // TODO: allow access only to the admin
        //     return Ok(router
        //         .route("/metrics", get(move || ready(prometheus_recorder.render())))
        //         .route_layer(middleware::from_fn(track_metrics)));
        // }

        Ok(router)
    }

    pub async fn run_server(self) -> Result<()> {
        tracing::info!(address = ?self.socket, "Start server");

        // TODO: implement graceful shutdown
        // let server: serve::Serve<axum::routing::IntoMakeService<Router>, Router> = serve(
        //     TcpListener::bind(self.socket).await?,
        //     self.create_router()?.into_make_service(),
        // );

        let server = Server::bind(&self.socket).serve(self.create_router()?.into_make_service());

        server.await.map_err(anyhow::Error::from)
    }
}

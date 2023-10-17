use {
    anyhow::Result,
    axum::{
        extract::State,
        http::StatusCode,
        routing::{get, put},
        Json, Router, Server,
    },
    serde::Serialize,
    std::{
        future::Future,
        sync::{Arc, Mutex},
        time::Duration,
    },
    tokio::{
        task::{spawn, JoinHandle},
        time::sleep,
    },
};

use super::{state::IndexerState, Indexer};

impl Indexer {
    async fn shutdown_signal(&self) -> impl Future<Output = ()> {
        let shared_state = self.state.clone();
        async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                if matches!(*shared_state.lock().unwrap(), IndexerState::Stopped { .. }) {
                    break;
                }
            }
        }
    }

    fn create_router(&self) -> Router {
        Router::new()
            .route("/state", get(get_state))
            .route("/stop", put(stop))
            .with_state(self.state.clone())
    }

    pub async fn spawn_server(&self, addr: &str) -> Result<JoinHandle<Result<(), hyper::Error>>> {
        tracing::debug!(?addr, "Start server");

        let server = Server::bind(&addr.parse()?)
            .serve(self.create_router().into_make_service())
            .with_graceful_shutdown(self.shutdown_signal().await);

        Ok(spawn(server))
    }
}

#[derive(Serialize)]
struct GetStateOutput {
    state: IndexerState,
}

async fn get_state(
    State(shared_state): State<Arc<Mutex<IndexerState>>>,
) -> Result<Json<GetStateOutput>, axum::http::StatusCode> {
    let state = shared_state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .clone();

    Ok(Json(GetStateOutput { state }))
}

#[derive(Serialize)]
struct StopOutput {
    success: bool,
}

async fn stop(
    State(shared_state): State<Arc<Mutex<IndexerState>>>,
) -> Result<Json<StopOutput>, axum::http::StatusCode> {
    shared_state
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .transition(IndexerState::Stopped {
            message: "Stopped by user".to_string(),
        });

    Ok(Json(StopOutput { success: true }))
}

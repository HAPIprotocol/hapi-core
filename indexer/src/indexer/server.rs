use {
    anyhow::Result,
    axum::{
        extract::State,
        routing::{get, put},
        Json, Router, Server,
    },
    serde::Serialize,
    std::{future::Future, sync::Arc, time::Duration},
    tokio::{
        sync::Mutex,
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
                if matches!(*shared_state.lock().await, IndexerState::Stopped { .. }) {
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

    pub async fn spawn_server(&self, addr: &str) -> Result<JoinHandle<Result<()>>> {
        tracing::debug!(?addr, "Start server");

        let server = Server::bind(&addr.parse()?)
            .serve(self.create_router().into_make_service())
            .with_graceful_shutdown(self.shutdown_signal().await);

        Ok(spawn(
            async move { server.await.map_err(anyhow::Error::from) },
        ))
    }
}

#[derive(Serialize)]
struct GetStateOutput {
    state: IndexerState,
}

async fn get_state(State(shared_state): State<Arc<Mutex<IndexerState>>>) -> Json<GetStateOutput> {
    let state = shared_state.lock().await.clone();

    Json(GetStateOutput { state })
}

#[derive(Serialize)]
struct StopOutput {
    success: bool,
}

async fn stop(State(shared_state): State<Arc<Mutex<IndexerState>>>) -> Json<StopOutput> {
    shared_state.lock().await.transition(IndexerState::Stopped {
        message: "Stopped by user".to_string(),
    });

    Json(StopOutput { success: true })
}

use axum::{extract::State, response::IntoResponse, Json};

use crate::error::AppError;

use super::AppState;

pub(crate) async fn indexer(state: State<AppState>) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Received indexer request");
    let _db = &state.database_conn;

    const MESSAGE: &str = "Indexer is not implemented yet";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok(Json(json_response))
}

pub(crate) async fn indexer_heartbeat(state: State<AppState>, id: String) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Received indexer heartbeat");
    let _db = &state.database_conn;

    let msg = format!("Indexer {} heartbeat received", id);

    let json_response = serde_json::json!({
        "status": "success",
        "message": msg
    });

    Ok(Json(json_response))
}
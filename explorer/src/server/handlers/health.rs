use axum::{http::StatusCode, response::IntoResponse};

/// Handle health Requests
pub(crate) async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

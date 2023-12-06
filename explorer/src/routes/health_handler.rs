use axum::{http::StatusCode, response::IntoResponse};

pub(crate) async fn health() -> impl IntoResponse {
    StatusCode::OK
}
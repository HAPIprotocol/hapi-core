use axum::{extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;

pub(crate) async fn entities(state: State<DatabaseConnection>) -> impl IntoResponse {
    unimplemented!()
}

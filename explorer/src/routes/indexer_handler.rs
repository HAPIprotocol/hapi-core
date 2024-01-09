use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};
use uuid::Uuid;

use crate::{entity::indexer, error::AppError};

use super::AppState;

const DEFAULT_PAGE_SIZE: u64 = 25;

#[derive(serde::Deserialize)]
pub struct PaginationParams {
    page: Option<u64>,
    page_size: Option<u64>,
}

pub(crate) async fn indexer(
    state: State<AppState>,
    pagination: Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Received indexer request");
    let db = &state.database_conn;

    let page = pagination.page.unwrap_or_default();
    let page_size = pagination.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let indexers_count = indexer::Entity::find().count(db).await?;

    let result = indexer::Entity::find()
        .paginate(db, page_size)
        .fetch_page(page)
        .await?;

    let json_response = serde_json::json!({
        "data": result,
        "meta": {
            "total": indexers_count,
            "page": page,
            "page_size": page_size,
        }
    });

    Ok(Json(json_response))
}

pub(crate) async fn indexer_heartbeat(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    cursor: String,
) -> Result<impl IntoResponse, AppError> {
    let db = &state.database_conn;

    indexer::ActiveModel {
        id: Set(id),
        last_heartbeat: Set(chrono::Utc::now().naive_utc()),
        cursor: Set(cursor),
        ..Default::default()
    }
    .update(db)
    .await?;

    let json_response = serde_json::json!({
        "status": "success"
    });

    Ok(Json(json_response))
}

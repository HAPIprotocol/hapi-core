use {
    axum::{
        extract::{Json, State},
        http::StatusCode,
    },
    hapi_core::client::{
        entities::{
            address::Address as AddressPayload, asset::Asset as AssetPayload,
            case::Case as CasePayload, reporter::Reporter as ReporterPayload,
        },
        events::EventName,
    },
    hapi_indexer::{PushData, PushPayload},
    sea_orm::DatabaseConnection,
    tracing::instrument,
};

use crate::{
    application::AppState,
    entity::{address, asset, case, reporter},
    error::AppError,
    observability::{
        update_address_metrics, update_asset_metrics, update_case_metrics, update_reporter_metrics,
        MetricOp,
    },
    service::{get_network_id, EntityMutation, EntityQuery},
};

/// Handle events Requests
#[instrument(level = "info", skip(state))]
pub(crate) async fn event_handler(
    state: State<AppState>,
    Json(payload): Json<PushPayload>,
) -> Result<StatusCode, AppError> {
    tracing::info!(event = ?payload.event, "Received event");
    let event_name = payload.event.name;
    let timestamp = payload.event.timestamp;
    let db = &state.database_conn;

    let network_id = get_network_id(
        db,
        payload.network_data.network.into(),
        payload.network_data.chain_id,
    )
    .await?;

    match payload.data {
        PushData::Address(address) => {
            process_address_payload(address, event_name, db, network_id, timestamp).await
        }
        PushData::Asset(asset) => {
            process_asset_payload(asset, event_name, db, network_id, timestamp).await
        }
        PushData::Case(case) => {
            process_case_payload(case, event_name, db, network_id, timestamp).await
        }
        PushData::Reporter(reporter) => {
            process_reporter_payload(reporter, event_name, db, network_id, timestamp).await
        }
    }
}

#[instrument(level = "trace", skip(db))]
async fn process_address_payload(
    address: AddressPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: String,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(address = ?address, "Received address");

    let address = match event_name {
        EventName::CreateAddress => {
            EntityMutation::create_entity::<address::ActiveModel, _>(
                db, address, network_id, timestamp,
            )
            .await?
        }
        EventName::UpdateAddress => {
            let old = EntityQuery::find_entity_by_id::<address::Entity, _>(
                db,
                (network_id.clone(), address.address.clone()),
            )
            .await?
            .ok_or(AppError::invalid_request("This address does not exist"))?;

            let new = EntityMutation::update_entity::<address::ActiveModel, _>(
                db, address, network_id, timestamp,
            )
            .await?;

            update_address_metrics(old, MetricOp::Decrement);

            new
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with address payload: {event_name}",
            )));
        }
    };

    update_address_metrics(address, MetricOp::Increment);

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_asset_payload(
    asset: AssetPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: String,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(asset = ?asset, "Received asset");

    let asset = match event_name {
        EventName::CreateAsset => {
            EntityMutation::create_entity::<asset::ActiveModel, _>(db, asset, network_id, timestamp)
                .await?
        }
        EventName::UpdateAsset => {
            let old = EntityQuery::find_entity_by_id::<asset::Entity, _>(
                db,
                (
                    network_id.clone(),
                    asset.address.clone(),
                    asset.asset_id.to_string(),
                ),
            )
            .await?
            .ok_or(AppError::invalid_request("This asset does not exist"))?;

            let new = EntityMutation::update_entity::<asset::ActiveModel, _>(
                db, asset, network_id, timestamp,
            )
            .await?;

            update_asset_metrics(old, MetricOp::Decrement);

            new
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with asset payload: {event_name}"
            )));
        }
    };

    update_asset_metrics(asset, MetricOp::Increment);

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_case_payload(
    case: CasePayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: String,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(case = ?case, "Received case");

    let case = match event_name {
        EventName::CreateCase => {
            EntityMutation::create_entity::<case::ActiveModel, _>(db, case, network_id, timestamp)
                .await?
        }
        EventName::UpdateCase => {
            let old = EntityQuery::find_entity_by_id::<case::Entity, _>(
                db,
                (network_id.clone(), case.id),
            )
            .await?
            .ok_or(AppError::invalid_request("This case does not exist"))?;

            let new = EntityMutation::update_entity::<case::ActiveModel, _>(
                db, case, network_id, timestamp,
            )
            .await?;

            update_case_metrics(old, MetricOp::Decrement);

            new
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with case payload: {event_name}",
            )));
        }
    };

    update_case_metrics(case, MetricOp::Increment);

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_reporter_payload(
    reporter: ReporterPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: String,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(reporter = ?reporter, "Received reporter");

    let reporter = match event_name {
        EventName::CreateReporter => {
            EntityMutation::create_entity::<reporter::ActiveModel, _>(
                db, reporter, network_id, timestamp,
            )
            .await?
        }
        EventName::UpdateReporter
        | EventName::ActivateReporter
        | EventName::DeactivateReporter
        | EventName::Unstake => {
            let old = EntityQuery::find_entity_by_id::<reporter::Entity, _>(
                db,
                (network_id.clone(), reporter.id),
            )
            .await?
            .ok_or(AppError::invalid_request("This reporter does not exist"))?;

            let new = EntityMutation::update_entity::<reporter::ActiveModel, _>(
                db, reporter, network_id, timestamp,
            )
            .await?;

            update_reporter_metrics(old, MetricOp::Decrement);

            new
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with reporter payload: {event_name}"
            )));
        }
    };

    update_reporter_metrics(reporter, MetricOp::Increment);

    Ok(StatusCode::OK)
}

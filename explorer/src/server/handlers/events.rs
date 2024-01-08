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
    std::sync::Arc,
    tracing::instrument,
};

use crate::{
    entity::{address, asset, case, reporter, types::NetworkBackend},
    error::AppError,
    service::EntityMutation,
};

/// Handle events Requests
#[instrument(level = "info", skip(db))]
pub(crate) async fn event_handler(
    db: State<Arc<DatabaseConnection>>,
    Json(payload): Json<PushPayload>,
) -> Result<StatusCode, AppError> {
    tracing::info!(event = ?payload.event, "Received event");
    let event_name = payload.event.name;
    let timestamp = payload.event.timestamp;
    let network = payload.network.into();

    match payload.data {
        PushData::Address(address) => {
            process_address_payload(address, event_name, &db, network, timestamp).await
        }
        PushData::Asset(asset) => {
            process_asset_payload(asset, event_name, &db, network, timestamp).await
        }
        PushData::Case(case) => {
            process_case_payload(case, event_name, &db, network, timestamp).await
        }
        PushData::Reporter(reporter) => {
            process_reporter_payload(reporter, event_name, &db, network, timestamp).await
        }
    }
}

#[instrument(level = "trace", skip(db))]
async fn process_address_payload(
    address: AddressPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network: NetworkBackend,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(address = ?address, "Received address");

    match event_name {
        EventName::CreateAddress => {
            EntityMutation::create_entity::<address::ActiveModel, _>(
                db, address, network, timestamp,
            )
            .await?;
        }
        EventName::UpdateAddress => {
            EntityMutation::update_entity::<address::ActiveModel, _>(
                db, address, network, timestamp,
            )
            .await?;
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with address payload: {:?}",
                event_name
            )));
        }
    }

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_asset_payload(
    asset: AssetPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network: NetworkBackend,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(asset = ?asset, "Received asset");

    match event_name {
        EventName::CreateAsset => {
            EntityMutation::create_entity::<asset::ActiveModel, _>(db, asset, network, timestamp)
                .await?;
        }
        EventName::UpdateAsset => {
            EntityMutation::update_entity::<asset::ActiveModel, _>(db, asset, network, timestamp)
                .await?;
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with asset payload: {:?}",
                event_name
            )));
        }
    }

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_case_payload(
    case: CasePayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network: NetworkBackend,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(case = ?case, "Received case");

    match event_name {
        EventName::CreateCase => {
            EntityMutation::create_entity::<case::ActiveModel, _>(db, case, network, timestamp)
                .await?;
        }
        EventName::UpdateCase => {
            EntityMutation::update_entity::<case::ActiveModel, _>(db, case, network, timestamp)
                .await?;
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with case payload: {:?}",
                event_name
            )));
        }
    }

    Ok(StatusCode::OK)
}

#[instrument(level = "trace", skip(db))]
async fn process_reporter_payload(
    reporter: ReporterPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network: NetworkBackend,
    timestamp: u64,
) -> Result<StatusCode, AppError> {
    tracing::info!(reporter = ?reporter, "Received reporter");

    match event_name {
        EventName::CreateReporter => {
            EntityMutation::create_entity::<reporter::ActiveModel, _>(
                db, reporter, network, timestamp,
            )
            .await?;
        }
        EventName::UpdateReporter
        | EventName::ActivateReporter
        | EventName::DeactivateReporter
        | EventName::Unstake => {
            EntityMutation::update_entity::<reporter::ActiveModel, _>(
                db, reporter, network, timestamp,
            )
            .await?;
        }
        _ => {
            return Err(AppError::invalid_request(&format!(
                "Received unexpected event with reporter payload: {:?}",
                event_name
            )));
        }
    }

    Ok(StatusCode::OK)
}

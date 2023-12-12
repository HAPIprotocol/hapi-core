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
    uuid::Uuid,
};

use crate::{
    entity::{address, asset, case, reporter},
    error::AppError,
    service::Mutation,
};

pub(crate) async fn events(
    db: State<DatabaseConnection>,
    Json(payload): Json<PushPayload>,
) -> Result<StatusCode, AppError> {
    tracing::info!(event = ?payload.event, "Received event");
    let event_name = payload.event.name;
    let network_id = payload.network_id;

    match payload.data {
        PushData::Address(address) => {
            process_address_payload(address, event_name, &db, network_id).await
        }
        PushData::Asset(asset) => process_asset_payload(asset, event_name, &db, network_id).await,
        PushData::Case(case) => process_case_payload(case, event_name, &db, network_id).await,
        PushData::Reporter(reporter) => {
            process_reporter_payload(reporter, event_name, &db, network_id).await
        }
    }
}

// TODO: add tracing
async fn process_address_payload(
    address: AddressPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: Uuid,
) -> Result<StatusCode, AppError> {
    tracing::info!(address = ?address, "Received address");

    match event_name {
        EventName::CreateAddress => {
            Mutation::create_entity::<address::ActiveModel, _>(db, address, network_id).await?;
        }
        EventName::UpdateAddress => {
            Mutation::update_entity::<address::ActiveModel, _>(db, address, network_id).await?;
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

async fn process_asset_payload(
    asset: AssetPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: Uuid,
) -> Result<StatusCode, AppError> {
    tracing::info!(asset = ?asset, "Received asset");

    match event_name {
        EventName::CreateAsset => {
            Mutation::create_entity::<asset::ActiveModel, _>(db, asset, network_id).await?;
        }
        EventName::UpdateAsset => {
            Mutation::update_entity::<asset::ActiveModel, _>(db, asset, network_id).await?;
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

async fn process_case_payload(
    case: CasePayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: Uuid,
) -> Result<StatusCode, AppError> {
    tracing::info!(case = ?case, "Received case");

    match event_name {
        EventName::CreateCase => {
            Mutation::create_entity::<case::ActiveModel, _>(db, case, network_id).await?;
        }
        EventName::UpdateCase => {
            Mutation::update_entity::<case::ActiveModel, _>(db, case, network_id).await?;
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

async fn process_reporter_payload(
    reporter: ReporterPayload,
    event_name: EventName,
    db: &DatabaseConnection,
    network_id: Uuid,
) -> Result<StatusCode, AppError> {
    tracing::info!(reporter = ?reporter, "Received reporter");

    match event_name {
        EventName::CreateReporter => {
            Mutation::create_entity::<reporter::ActiveModel, _>(db, reporter, network_id).await?;
        }
        EventName::UpdateReporter
        | EventName::ActivateReporter
        | EventName::DeactivateReporter
        | EventName::Unstake => {
            Mutation::update_entity::<reporter::ActiveModel, _>(db, reporter, network_id).await?;
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

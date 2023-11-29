use anyhow::Result;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use hapi_core::client::{
    entities::{
        address::Address as AddressPayload, asset::Asset as AssetPayload,
        case::Case as CasePayload, reporter::Reporter as ReporterPayload,
    },
    events::EventName,
};
use hapi_indexer::{PushData, PushPayload};
use sea_orm::DatabaseConnection;

use crate::entity::{address, asset, case, reporter};
use crate::service::Mutation;

pub(crate) async fn entities(
    db: State<DatabaseConnection>,
    Json(payload): Json<PushPayload>,
) -> impl IntoResponse {
    tracing::info!(event = ?payload.event, "Received event");
    let event_name = payload.event.name;

    match payload.data {
        PushData::Address(address) => process_address_payload(address, event_name, &db)
            .await
            .unwrap(),
        PushData::Asset(asset) => process_asset_payload(asset, event_name, &db).await.unwrap(),
        PushData::Case(case) => process_case_payload(case, event_name, &db).await.unwrap(),
        PushData::Reporter(reporter) => process_reporter_payload(reporter, event_name, &db)
            .await
            .unwrap(),
    }
}

async fn process_address_payload(
    address: AddressPayload,
    event_name: EventName,
    db: &DatabaseConnection,
) -> Result<StatusCode> {
    tracing::info!(address = ?address, "Received address");

    match event_name {
        EventName::CreateAddress => {
            Mutation::create_entity::<address::ActiveModel, _>(&db, address)
                .await
                .unwrap();
        }
        EventName::UpdateAddress => {
            Mutation::update_entity::<address::ActiveModel, _>(&db, address)
                .await
                .unwrap();
        }
        _ => {
            tracing::error!(event_name = ?event_name, "Received unexpected event with address payload");
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    Ok(StatusCode::OK)
}

async fn process_asset_payload(
    asset: AssetPayload,
    event_name: EventName,
    db: &DatabaseConnection,
) -> Result<StatusCode> {
    tracing::info!(asset = ?asset, "Received asset");

    match event_name {
        EventName::CreateAsset => {
            Mutation::create_entity::<asset::ActiveModel, _>(&db, asset)
                .await
                .unwrap();
        }
        EventName::UpdateAsset => {
            Mutation::update_entity::<asset::ActiveModel, _>(&db, asset)
                .await
                .unwrap();
        }
        _ => {
            tracing::error!(event_name = ?event_name, "Received unexpected event with asset payload");
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    Ok(StatusCode::OK)
}

async fn process_case_payload(
    case: CasePayload,
    event_name: EventName,
    db: &DatabaseConnection,
) -> Result<StatusCode> {
    tracing::info!(case = ?case, "Received case");

    match event_name {
        EventName::CreateCase => {
            Mutation::create_entity::<case::ActiveModel, _>(&db, case)
                .await
                .unwrap();
        }
        EventName::UpdateCase => {
            Mutation::update_entity::<case::ActiveModel, _>(&db, case)
                .await
                .unwrap();
        }
        _ => {
            tracing::error!(event_name = ?event_name, "Received unexpected event with case payload");
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    Ok(StatusCode::OK)
}

async fn process_reporter_payload(
    reporter: ReporterPayload,
    event_name: EventName,
    db: &DatabaseConnection,
) -> Result<StatusCode> {
    tracing::info!(reporter = ?reporter, "Received reporter");

    match event_name {
        EventName::CreateReporter => {
            Mutation::create_entity::<reporter::ActiveModel, _>(&db, reporter)
                .await
                .unwrap();
        }
        EventName::UpdateReporter
        | EventName::ActivateReporter
        | EventName::DeactivateReporter
        | EventName::Unstake => {
            Mutation::update_entity::<reporter::ActiveModel, _>(&db, reporter)
                .await
                .unwrap();
        }
        _ => {
            tracing::error!(event_name = ?event_name, "Received unexpected event with reporter payload");
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    Ok(StatusCode::OK)
}

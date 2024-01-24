use super::replacer;
use crate::helpers::{FromTestPayload, RequestSender, TestApp, TestData};

use {
    hapi_core::client::{entities::asset::Asset, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_ASSET_QUERY: &str = "
    query GetAsset($address: String!, $id: String!, $networkId: String!) {
        getAsset(address: $address, id: $id, networkId: $networkId) {
            networkId
            address
            id
            caseId
            reporterId
            risk
            category
            confirmations
            createdAt
            updatedAt
        }
    }
";
const GET_MANY_ASSETS: &str = "
    query GetManyAssets(
        $input: AssetInput!
    ) {
        getManyAssets(
            input: $input
        ) {
            data {
                networkId
                address
                id
                caseId
                reporterId
                risk
                category
                confirmations
                createdAt
                updatedAt
            }
            total
            pageCount
        }
    }
";

impl FromTestPayload for TestData<Asset> {
    fn from_payload(payload: &PushPayload, network_id: &str) -> TestData<Asset> {
        let entity = match &payload.data {
            PushData::Asset(asset) => asset,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network_id: network_id.to_string(),
        }
    }
}

fn check_asset(asset: &TestData<Asset>, value: &Value) {
    assert_eq!(value["networkId"], asset.network_id);

    let payload = &asset.data;
    assert_eq!(value["address"], payload.address);
    assert_eq!(value["id"], payload.asset_id.to_string());
    assert_eq!(value["caseId"], payload.case_id.to_string());
    assert_eq!(value["reporterId"], payload.reporter_id.to_string());
    assert_eq!(value["risk"], payload.risk);
    assert_eq!(
        replacer(&value["category"]),
        payload.category.to_string().to_lowercase()
    );
    assert_eq!(value["confirmations"], payload.confirmations.to_string());
}

#[tokio::test]
async fn get_asset_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .global_setup::<Asset>(&sender, EventName::UpdateAsset)
        .await;

    for payload in assets {
        let response = sender
            .send_graphql(
                GET_ASSET_QUERY,
                json!({
                    "address": payload.data.address,
                    "id": payload.data.asset_id,
                    "networkId": payload.network_id
                }),
            )
            .await
            .unwrap();

        let asset = &response["getAsset"];
        check_asset(&payload, asset);
    }
}

#[tokio::test]
async fn get_many_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .global_setup::<Asset>(&sender, EventName::UpdateAsset)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_ASSETS,
            json!({
            "input":
            {
              "ordering": "ASC",
              "orderingCondition": "UPDATED_AT",
            }

            }),
        )
        .await
        .unwrap();

    let assets_response = &response["getManyAssets"];
    assert_eq!(assets_response["total"], assets.len());

    for (index, asset) in assets_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let payload = assets.get(index).expect("Invalid index");
        check_asset(payload, asset)
    }
}

#[tokio::test]
async fn get_filtered_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .global_setup::<Asset>(&sender, EventName::UpdateAsset)
        .await;

    for payload in assets {
        let response = sender
            .send_graphql(
                GET_MANY_ASSETS,
                json!({
                "input":
                {
                    "filtering": {
                        "reporterId": payload.data.reporter_id.to_string(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }

                }),
            )
            .await
            .unwrap();

        let assets_response = &response["getManyAssets"];
        assert_eq!(assets_response["total"], 1);

        let asset = assets_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_asset(&payload, asset)
    }
}

#[tokio::test]
async fn get_paginated_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .global_setup::<Asset>(&sender, EventName::UpdateAsset)
        .await;

    let payload = assets.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_ASSETS,
            json!({
            "input":
            {
                "ordering": "ASC",
                "orderingCondition": "UPDATED_AT",
                "pagination": {
                    "pageNum": assets.len() / page_size,
                    "pageSize": page_size
                }
            }
            }),
        )
        .await
        .unwrap();

    let assets_response = &response["getManyAssets"];
    assert_eq!(assets_response["total"], assets.len());
    assert_eq!(assets_response["pageCount"], assets.len() / page_size);

    let assets = assets_response["data"].as_array().expect("Empty response");

    assert_eq!(assets.len(), page_size);
    check_asset(&payload, assets.last().unwrap())
}

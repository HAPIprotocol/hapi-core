use crate::helpers::{RequestSender, TestApp};

use {
    hapi_core::{
        client::{entities::asset::Asset, events::EventName},
        HapiCoreNetwork,
    },
    hapi_indexer::PushData,
    serde_json::{json, Value},
};

const GET_ASSET_QUERY: &str = "
    query GetAsset($address: String!, $assetId: String!, $network: UUID!) {
        getAsset(address: $address, assetId: $assetId, network: $network) {
            network
            address
            assetId
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
                network
                address
                assetId
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

fn check_asset(payload: &Asset, value: &Value, network_id: &HapiCoreNetwork) {
    let replacer = |v: &Value| {
        v.to_string()
            .replace("\"", "")
            .replace("_", "")
            .to_lowercase()
    };

    assert_eq!(
        replacer(&value["network"]),
        network_id.to_string().to_lowercase()
    );
    assert_eq!(value["address"], payload.address);
    assert_eq!(value["assetId"], payload.asset_id.to_string());
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
        .setup_entities(&sender, EventName::UpdateAsset)
        .await;

    for (payload, network) in assets {
        let asset_payload = match payload {
            PushData::Asset(asset) => asset,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_ASSET_QUERY,
                json!({
                    "address": asset_payload.address,
                    "assetId": asset_payload.asset_id,
                    "network": network.to_string().to_uppercase()
                }),
            )
            .await;

        let asset = &response["getAsset"];
        check_asset(&asset_payload, asset, &network);
    }
}

#[tokio::test]
async fn get_many_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .setup_entities(&sender, EventName::UpdateAsset)
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
        .await;

    let assets_response = &response["getManyAssets"];
    assert_eq!(assets_response["total"], assets.len());

    for (index, asset) in assets_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let (payload, network) = assets.get(index).expect("Invalid index");
        let asset_payload = match payload {
            PushData::Asset(asset) => asset,
            _ => panic!("Invalid type"),
        };

        check_asset(asset_payload, asset, network)
    }
}

#[tokio::test]
async fn get_filtered_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .setup_entities(&sender, EventName::UpdateAsset)
        .await;

    for (payload, network) in assets {
        let asset_payload = match payload {
            PushData::Asset(asset) => asset,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_MANY_ASSETS,
                json!({
                "input":
                {
                    "filtering": {
                        "reporterId": asset_payload.reporter_id.to_string(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }

                }),
            )
            .await;

        let assets_response = &response["getManyAssets"];
        assert_eq!(assets_response["total"], 1);

        let asset = assets_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_asset(&asset_payload, asset, network)
    }
}

#[tokio::test]
async fn get_paginated_assets_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let assets = test_app
        .setup_entities(&sender, EventName::UpdateAsset)
        .await;

    let (payload, network) = assets.last().expect("Invalid index");
    let asset_payload = match payload {
        PushData::Asset(asset) => asset,
        _ => panic!("Invalid type"),
    };

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
        .await;

    let assets_response = &response["getManyAssets"];
    assert_eq!(assets_response["total"], assets.len());
    assert_eq!(assets_response["pageCount"], assets.len() / page_size);

    let assets = assets_response["data"].as_array().expect("Empty response");

    assert_eq!(assets.len(), page_size);
    check_asset(&asset_payload, assets.last().unwrap(), network)
}

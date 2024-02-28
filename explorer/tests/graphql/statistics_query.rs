use {
    chrono::{Datelike, Utc},
    hapi_core::client::{entities::case::Case, events::EventName},
    hapi_explorer::entity::statistics::CHART_LENGTH,
    hapi_indexer::{PushData, PushPayload},
    serde_json::Value,
    web3::types::U256,
};

use super::check_case;
use crate::helpers::{
    create_address_data, create_asset_data, create_reporter_data, RequestSender, TestApp,
};

const GET_DASHBOARD_QUERY: &str = "
    query GetDashboard {
        getDashboard {
            stakedByReporters
            totalReportersCount
        
            totalAddressesCount
            newWeeklyAddressCount
            lastAddedAddresses {
                networkId
                address
            }

            totalAssetCount
            newWeeklyAssetCount
            lastAddedAssets {
                networkId
                id
            }

            totalCaseCount
            newWeeklyCaseCount
            topCasesByAddress {
                networkId
                id
                name
                url
                status
                reporterId
                createdAt
                updatedAt
            }
            topCasesByAsset {
                networkId
                id
                name
                url
                status
                reporterId
                createdAt
                updatedAt
            }
        }
    }
";
const GET_CHARTS_QUERY: &str = "
    query GetCharts {
        getCharts {
            labels
            addresses
            assets
            cases
        }
    }
";

#[tokio::test]
async fn dashboard_statistics_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    let mut reporters_stake = U256::zero();
    let mut entity_count = cases.len();

    for (count, case) in cases.iter().enumerate() {
        let network = test_app.get_network(&case.network_id);

        entity_count += (0..count + 1).count();

        let address_data = (0..count + 1)
            .map(|_| {
                create_address_data(
                    case.data.reporter_id,
                    case.data.id,
                    &network.network,
                    network.model.chain_id.clone(),
                )
            })
            .collect::<Vec<PushPayload>>();

        let asset_data = (0..cases.len() - count)
            .map(|_| {
                create_asset_data(
                    case.data.reporter_id,
                    case.data.id,
                    &network.network,
                    network.model.chain_id.clone(),
                )
            })
            .collect::<Vec<PushPayload>>();

        let reporter_data = (0..cases.len() - count)
            .map(|_| create_reporter_data(&network.network, network.model.chain_id.clone()))
            .collect::<Vec<PushPayload>>();

        reporter_data.iter().for_each(|payload| {
            if let PushData::Reporter(reporter) = &payload.data {
                reporters_stake += reporter.stake.clone().into();
            }
        });

        test_app.send_events(&sender, &address_data).await;
        test_app.send_events(&sender, &asset_data).await;
        test_app.send_events(&sender, &reporter_data).await;
    }

    let response = sender
        .send_graphql(GET_DASHBOARD_QUERY, Value::Null)
        .await
        .unwrap();

    let dashboard_response = &response["getDashboard"];

    assert_eq!(
        dashboard_response["stakedByReporters"],
        reporters_stake.to_string()
    );
    assert_eq!(dashboard_response["totalReportersCount"], entity_count);

    assert_eq!(dashboard_response["totalAddressesCount"], entity_count);
    assert_eq!(dashboard_response["newWeeklyAddressCount"], entity_count);
    assert_eq!(
        dashboard_response["lastAddedAddresses"]
            .as_array()
            .unwrap()
            .len(),
        CHART_LENGTH
    );

    assert_eq!(dashboard_response["totalAssetCount"], entity_count);
    assert_eq!(dashboard_response["newWeeklyAssetCount"], entity_count);
    assert_eq!(
        dashboard_response["lastAddedAssets"]
            .as_array()
            .unwrap()
            .len(),
        CHART_LENGTH
    );

    assert_eq!(dashboard_response["totalCaseCount"], cases.len());
    assert_eq!(dashboard_response["newWeeklyCaseCount"], cases.len());

    for (index, case) in dashboard_response["topCasesByAddress"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let case_index = cases.len() - index - 1;
        check_case(cases.get(case_index).expect("Invalid index"), case)
    }

    for (index, case) in dashboard_response["topCasesByAsset"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        check_case(cases.get(index).expect("Invalid index"), case)
    }
}

#[tokio::test]
async fn charts_statistics_test() {
    let test_app = TestApp::start(None).await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    let entities_count = cases.len() as u64;

    let response = sender
        .send_graphql(GET_CHARTS_QUERY, Value::Null)
        .await
        .unwrap();

    let now = Utc::now();

    let current_year = now.iso_week().year();
    let current_week = now.iso_week().week();

    let past = now - chrono::Duration::weeks(CHART_LENGTH as i64 - 1);

    let past_year = past.iso_week().year();
    let past_week = past.iso_week().week();

    let charts_response = &response["getCharts"];

    let labels = charts_response["labels"]
        .as_array()
        .expect("Empty response");

    assert_eq!(labels.len(), CHART_LENGTH);
    assert_eq!(
        labels.first().unwrap().as_str().unwrap(),
        format!("{}:{}", past_year, past_week)
    );
    assert_eq!(
        labels.last().unwrap().as_str().unwrap(),
        format!("{}:{}", current_year, current_week)
    );

    let addresses = charts_response["addresses"]
        .as_array()
        .expect("Empty response");

    assert_eq!(addresses.first().unwrap().as_u64().unwrap(), 0);
    assert_eq!(addresses.last().unwrap().as_u64().unwrap(), entities_count);

    let assets = charts_response["assets"]
        .as_array()
        .expect("Empty response");

    assert_eq!(assets.first().unwrap().as_u64().unwrap(), 0);
    assert_eq!(assets.last().unwrap().as_u64().unwrap(), entities_count);

    let cases = charts_response["cases"].as_array().expect("Empty response");

    assert_eq!(cases.first().unwrap().as_u64().unwrap(), 0);
    assert_eq!(cases.last().unwrap().as_u64().unwrap(), entities_count);
}

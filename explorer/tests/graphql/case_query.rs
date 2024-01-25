use super::replacer;
use crate::helpers::{
    create_address_data, create_asset_data, FromTestPayload, RequestSender, TestApp, TestData,
};

use {
    hapi_core::client::{entities::case::Case, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_CASE_QUERY: &str = "
    query GetCase($id: UUID!, $networkId: String!) {
        getCase(id: $id, networkId: $networkId) {
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
";
const GET_MANY_CASES: &str = "
    query GetManyCases(
        $input: CaseInput!
    ) {
        getManyCases(
            input: $input
        ) {
            data {
                networkId
                id
                name
                url
                status
                reporterId
                createdAt
                updatedAt
            }
            total
            pageCount
        }
    }
";

impl FromTestPayload for TestData<Case> {
    fn from_payload(payload: &PushPayload, network_id: &str) -> TestData<Case> {
        let entity = match &payload.data {
            PushData::Case(case) => case,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network_id: network_id.to_string(),
        }
    }
}

fn check_case(case: &TestData<Case>, value: &Value) {
    assert_eq!(value["networkId"], case.network_id);

    let payload = &case.data;
    assert_eq!(value["id"], payload.id.to_string());
    assert_eq!(value["name"], payload.name);
    assert_eq!(value["url"], payload.url);
    assert_eq!(
        replacer(&value["status"]),
        payload.status.to_string().to_lowercase()
    );
    assert_eq!(value["reporterId"], payload.reporter_id.to_string());
}

#[tokio::test]
async fn get_case_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    for payload in cases {
        let response = sender
            .send_graphql(
                GET_CASE_QUERY,
                json!({
                    "id": payload.data.id,
                    "networkId": payload.network_id
                }),
            )
            .await
            .unwrap();

        let case = &response["getCase"];
        check_case(&payload, case);
    }
}

#[tokio::test]
async fn get_many_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_CASES,
            json!({
            "input":
            {
              "ordering": "ASC",
            }
            }),
        )
        .await
        .unwrap();

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());

    for (index, case) in cases_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        check_case(cases.get(index).expect("Invalid index"), case)
    }
}

#[tokio::test]
async fn get_ordered_desc_cases_by_address_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    for (count, case) in cases.iter().enumerate() {
        let network = test_app.get_network(&case.network_id);

        let test_data = (0..count + 1)
            .map(|_| {
                create_address_data(
                    case.data.reporter_id,
                    case.data.id,
                    &network.network,
                    network.model.chain_id.clone(),
                )
            })
            .collect::<Vec<PushPayload>>();

        test_app.send_events(&sender, &test_data).await;
    }

    let response = sender
        .send_graphql(
            GET_MANY_CASES,
            json!({
            "input":
            {
              "orderingCondition": "ADDRESS_COUNT",
            }
            }),
        )
        .await
        .unwrap();

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());

    for (index, case) in cases_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let case_index = cases.len() - index - 1;
        check_case(cases.get(case_index).expect("Invalid index"), case)
    }
}

#[tokio::test]
async fn get_ordered_asc_cases_by_asset_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    for (count, case) in cases.iter().enumerate() {
        let network = test_app.get_network(&case.network_id);

        let test_data = (0..count + 1)
            .map(|_| {
                create_asset_data(
                    case.data.reporter_id,
                    case.data.id,
                    &network.network,
                    network.model.chain_id.clone(),
                )
            })
            .collect::<Vec<PushPayload>>();

        test_app.send_events(&sender, &test_data).await;
    }

    let response = sender
        .send_graphql(
            GET_MANY_CASES,
            json!({
            "input":
            {
              "ordering": "ASC",
              "orderingCondition": "ASSET_COUNT",
            }
            }),
        )
        .await
        .unwrap();

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());

    for (index, case) in cases_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        check_case(cases.get(index).expect("Invalid index"), case)
    }
}

#[tokio::test]
async fn get_filtered_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    for case in cases {
        let response = sender
            .send_graphql(
                GET_MANY_CASES,
                json!({
                "input":
                {
                    "filtering": {
                        "reporterId": case.data.reporter_id.to_string(),
                    },
                    "ordering": "ASC",
                }
                }),
            )
            .await
            .unwrap();

        let cases_response = &response["getManyCases"];
        assert_eq!(cases_response["total"], 1);

        let payload = cases_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_case(&case, payload)
    }
}

#[tokio::test]
async fn get_paginated_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    let payload = cases.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_CASES,
            json!({
            "input":
            {
                "ordering": "ASC",
                "pagination": {
                    "pageNum": cases.len() / page_size,
                    "pageSize": page_size
                }
            }
            }),
        )
        .await
        .unwrap();

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());
    assert_eq!(cases_response["pageCount"], cases.len() / page_size);

    let cases = cases_response["data"].as_array().expect("Empty response");

    assert_eq!(cases.len(), page_size);
    check_case(&payload, cases.last().unwrap())
}

#[tokio::test]
async fn get_searched_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .global_setup::<Case>(&sender, EventName::UpdateCase)
        .await;

    for payload in cases {
        let response = sender
            .send_graphql(
                GET_MANY_CASES,
                json!({
                "input":
                {
                    "search" : &payload.network_id[0..payload.network_id.len() - 1],
                    "ordering": "ASC",
                }

                }),
            )
            .await
            .unwrap();

        let cases_response = &response["getManyCases"];
        assert_eq!(cases_response["total"], 1);

        let case = cases_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_case(&payload, case)
    }
}

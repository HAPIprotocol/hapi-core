use hapi_core::client::entities::{address::Address, asset::Asset};

use super::replacer;
use crate::helpers::{create_address_data, create_asset_data, RequestSender, TestApp, TestData};

use {
    hapi_core::client::{entities::case::Case, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_CASE_QUERY: &str = "
    query GetCase($caseId: UUID!, $network: UUID!) {
        getCase(caseId: $caseId, network: $network) {
            network
            caseId
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
                network
                caseId
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

impl From<PushPayload> for TestData<Case> {
    fn from(payload: PushPayload) -> Self {
        let entity = match &payload.data {
            PushData::Case(case) => case,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network: payload.network,
            indexer_id: payload.id,
        }
    }
}

fn check_case(case: &TestData<Case>, value: &Value) {
    assert_eq!(
        replacer(&value["network"]),
        case.network.to_string().to_lowercase()
    );
    let payload = &case.data;

    assert_eq!(value["caseId"], payload.id.to_string());
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
        .await;

    for payload in cases {
        let response = sender
            .send_graphql(
                GET_CASE_QUERY,
                json!({
                    "caseId": payload.data.id,
                    "network": payload.network.to_string().to_uppercase()
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_CASES,
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
        .await;

    for (count, case) in cases.iter().enumerate() {
        let test_data = (0..count + 1)
            .map(|_| {
                create_address_data(
                    case.data.reporter_id,
                    case.data.id,
                    case.network.clone(),
                    case.indexer_id,
                )
            })
            .collect::<Vec<PushPayload>>();

        let _ = test_app
            .setup_entities::<Address>(&sender, EventName::CreateAddress, Some(test_data))
            .await;
    }

    let response = sender
        .send_graphql(
            GET_MANY_CASES,
            json!({
            "input":
            {
              "ordering": "DESC",
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
        .await;

    for (count, case) in cases.iter().enumerate() {
        let test_data = (0..count + 1)
            .map(|_| {
                create_asset_data(
                    case.data.reporter_id,
                    case.data.id,
                    case.network.clone(),
                    case.indexer_id,
                )
            })
            .collect::<Vec<PushPayload>>();

        let _ = test_app
            .setup_entities::<Asset>(&sender, EventName::CreateAsset, Some(test_data))
            .await;
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
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
                    "orderingCondition": "UPDATED_AT",
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
        .setup_entities::<Case>(&sender, EventName::UpdateCase, None)
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
                "orderingCondition": "UPDATED_AT",
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

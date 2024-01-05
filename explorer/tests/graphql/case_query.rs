use crate::helpers::{RequestSender, TestApp};

use {
    hapi_core::{
        client::{entities::case::Case, events::EventName},
        HapiCoreNetwork,
    },
    hapi_indexer::PushData,
    serde_json::{json, Value},
};

const GET_CASE_QUERY: &str = "
    query GetCase($caseId: String!, $network: UUID!) {
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

fn check_case(payload: &Case, value: &Value, network_id: &HapiCoreNetwork) {
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
    assert_eq!(value["caseId"], payload.id.to_string());
    assert_eq!(value["name"], payload.name);
    assert_eq!(value["url"], payload.url);
    assert_eq!(replacer(&value["status"]), payload.status.to_string());
    assert_eq!(value["reporterId"], payload.reporter_id.to_string());
}

#[tokio::test]
async fn get_case_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .setup_entities(&sender, EventName::UpdateCase)
        .await;

    for (payload, network) in cases {
        let cases_payload = match payload {
            PushData::Case(case) => case,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_CASE_QUERY,
                json!({
                    "caseId": cases_payload.id,
                    "network": network.to_string().to_uppercase()
                }),
            )
            .await;

        let case = &response["getCase"];
        check_case(&cases_payload, case, &network);
    }
}

#[tokio::test]
async fn get_many_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .setup_entities(&sender, EventName::UpdateCase)
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
        .await;

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());

    for (index, case) in cases_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let (payload, network) = cases.get(index).expect("Invalid index");
        let cases_payload = match payload {
            PushData::Case(case) => case,
            _ => panic!("Invalid type"),
        };

        check_case(cases_payload, case, network)
    }
}

#[tokio::test]
async fn get_filtered_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .setup_entities(&sender, EventName::UpdateCase)
        .await;

    for (payload, network) in cases {
        let cases_payload = match payload {
            PushData::Case(case) => case,
            _ => panic!("Invalid type"),
        };

        let response = sender
            .send_graphql(
                GET_MANY_CASES,
                json!({
                "input":
                {
                    "filtering": {
                        "reporterId": cases_payload.reporter_id.to_string(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }
                }),
            )
            .await;

        let cases_response = &response["getManyCases"];
        assert_eq!(cases_response["total"], 1);

        let case = cases_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_case(&cases_payload, case, network)
    }
}

#[tokio::test]
async fn get_paginated_cases_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let cases = test_app
        .setup_entities(&sender, EventName::UpdateCase)
        .await;

    let (payload, network) = cases.last().expect("Invalid index");
    let cases_payload = match payload {
        PushData::Case(case) => case,
        _ => panic!("Invalid type"),
    };

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
        .await;

    let cases_response = &response["getManyCases"];
    assert_eq!(cases_response["total"], cases.len());
    assert_eq!(cases_response["pageCount"], cases.len() / page_size);

    let cases = cases_response["data"].as_array().expect("Empty response");

    assert_eq!(cases.len(), page_size);
    check_case(&cases_payload, cases.last().unwrap(), network)
}

use super::replacer;
use crate::helpers::{RequestSender, TestApp, TestData};

use {
    hapi_core::client::{entities::reporter::Reporter, events::EventName},
    hapi_indexer::{PushData, PushPayload},
    serde_json::{json, Value},
};

const GET_REPORTER_QUERY: &str = "
    query GetReporter($reporterId: UUID!, $network: UUID!) {
        getReporter(reporterId: $reporterId, network: $network) {
            network
            reporterId
            account
            role
            status
            name
            url
            stake
            unlockTimestamp
            createdAt
            updatedAt
        }
    }
";
const GET_MANY_REPORTERS: &str = "
    query GetManyReporters(
        $input: ReporterInput!
    ) {
        getManyReporters(
            input: $input
        ) {
            data {
                network
                reporterId
                account
                role
                status
                name
                url
                stake
                unlockTimestamp
                createdAt
                updatedAt
            }
            total
            pageCount
        }
    }
";

impl From<PushPayload> for TestData<Reporter> {
    fn from(payload: PushPayload) -> Self {
        let entity = match &payload.data {
            PushData::Reporter(reporter) => reporter,
            _ => panic!("Invalid type"),
        };

        Self {
            data: entity.to_owned(),
            network: payload.network,
            indexer_id: payload.id,
        }
    }
}

fn check_reporter(reporter: &TestData<Reporter>, value: &Value) {
    assert_eq!(
        replacer(&value["network"]),
        reporter.network.to_string().to_lowercase()
    );

    let payload = &reporter.data;
    assert_eq!(value["reporterId"], payload.id.to_string());
    assert_eq!(value["account"], payload.account);
    assert_eq!(
        replacer(&value["role"]),
        payload.role.to_string().to_lowercase()
    );
    assert_eq!(
        replacer(&value["status"]),
        payload.status.to_string().to_lowercase()
    );
    assert_eq!(value["name"], payload.name);
    assert_eq!(value["url"], payload.url);
    assert_eq!(value["stake"], payload.stake.to_string());
    assert_eq!(
        value["unlockTimestamp"],
        payload.unlock_timestamp.to_string()
    );
}

#[tokio::test]
async fn get_reporter_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let reporters = test_app
        .setup_entities::<Reporter>(&sender, EventName::UpdateReporter, None)
        .await;

    for payload in reporters {
        let response = sender
            .send_graphql(
                GET_REPORTER_QUERY,
                json!({
                    "reporterId": payload.data.id,
                    "network": payload.network.to_string().to_uppercase()
                }),
            )
            .await
            .unwrap();

        let reporter = &response["getReporter"];
        check_reporter(&payload, reporter);
    }
}

#[tokio::test]
async fn get_many_reporters_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let reporters = test_app
        .setup_entities::<Reporter>(&sender, EventName::UpdateReporter, None)
        .await;

    let response = sender
        .send_graphql(
            GET_MANY_REPORTERS,
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

    let reporters_response = &response["getManyReporters"];
    assert_eq!(reporters_response["total"], reporters.len());

    for (index, reporter) in reporters_response["data"]
        .as_array()
        .expect("Empty response")
        .iter()
        .enumerate()
    {
        let payload = reporters.get(index).expect("Invalid index");

        check_reporter(payload, reporter)
    }
}

#[tokio::test]
async fn get_filtered_reporters_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let reporters = test_app
        .setup_entities::<Reporter>(&sender, EventName::UpdateReporter, None)
        .await;

    for payload in reporters {
        let response = sender
            .send_graphql(
                GET_MANY_REPORTERS,
                json!({
                "input":
                {
                    "filtering": {
                        "network": payload.network.to_string().to_uppercase(),
                    },
                    "ordering": "ASC",
                    "orderingCondition": "UPDATED_AT",
                }
                }),
            )
            .await
            .unwrap();

        let reporters_response = &response["getManyReporters"];
        assert_eq!(reporters_response["total"], 1);

        let reporter = reporters_response["data"]
            .as_array()
            .expect("Empty response")
            .first()
            .unwrap();

        check_reporter(&payload, reporter)
    }
}

#[tokio::test]
async fn get_paginated_reporters_test() {
    let test_app = TestApp::start().await;
    let sender = RequestSender::new(test_app.server_addr.clone());
    let reporters = test_app
        .setup_entities::<Reporter>(&sender, EventName::UpdateReporter, None)
        .await;

    let payload = reporters.last().expect("Invalid index");

    let page_size = 2;
    let response = sender
        .send_graphql(
            GET_MANY_REPORTERS,
            json!({
            "input":
            {
                "ordering": "ASC",
                "orderingCondition": "UPDATED_AT",
                "pagination": {
                    "pageNum": reporters.len() / page_size,
                    "pageSize": page_size
                }
            }
            }),
        )
        .await
        .unwrap();

    let reporters_response = &response["getManyReporters"];
    assert_eq!(reporters_response["total"], reporters.len());
    assert_eq!(reporters_response["pageCount"], reporters.len() / page_size);

    let reporters = reporters_response["data"]
        .as_array()
        .expect("Empty response");

    assert_eq!(reporters.len(), page_size);
    check_reporter(&payload, reporters.last().unwrap())
}

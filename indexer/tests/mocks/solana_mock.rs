use {
    hapi_core::client::solana::{test_helpers::create_test_tx, InstructionData},
    hapi_indexer::SOLANA_BATCH_SIZE,
    mockito::{Matcher, Server},
    serde_json::{json, Value},
    solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
};

use super::{RpcMock, TestEvent};

pub const PROGRAM_ID: &str = "39WzZqJgkK2QuQxV9jeguKRgHE65Q3HywqPwBzdrKn2B";
pub const REPORTER: &str = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
pub const CASE: &str = "DTDk9GEQoVibTuHmTfDUwHehkH4WYd5fpawPfayGRVdi";
pub const ADDRESS: &str = "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC";
pub const ASSET: &str = "5f2iaDyv4yzTudiNc1XR2EkEW5NtVbfZpqmjZ3fhFtaX";

pub struct SolanaMock;

impl RpcMock for SolanaMock {
    fn fetching_jobs_mock(server: &mut Server, batches: &[&[&str]]) {
        SolanaMock::mock_batches(server, batches);
    }

    fn processing_jobs_mock(server: &mut Server, events: &[TestEvent]) {
        SolanaMock::mock_transactions(server, events);
        SolanaMock::mock_accounts(server);
    }
}

impl SolanaMock {
    fn get_transaction(event: &TestEvent) -> EncodedConfirmedTransactionWithStatusMeta {
        // TODO: what about asset?
        let account_keys = vec![
            String::from(PROGRAM_ID),
            String::default(),
            String::default(),
            String::from(REPORTER),
            String::from(CASE),
            String::from(ADDRESS),
        ];

        let test_data = event
            .events
            .iter()
            .map(|n| (n.as_str(), InstructionData::Raw(String::from("Some data"))))
            .collect();

        create_test_tx(&test_data, event.signature.clone(), account_keys)
    }

    fn mock_batches(server: &mut Server, batches: &[&[&str]]) {
        let mut until = None;

        for batch in batches {
            let signatures: Vec<Value> = batch
                .iter()
                .map(|signature| {
                    json!({
                        "signature": signature,
                        "slot": 100,
                    })
                })
                .collect();

            let response = json!({
                "jsonrpc": "2.0",
                "result": signatures,
                "id": 1
            });

            server
                .mock("POST", "/")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&response.to_string())
                .match_body(Matcher::PartialJson(json!({
                    "method": "getSignaturesForAddress",
                    "params": [ PROGRAM_ID,
                    {
                      "limit": SOLANA_BATCH_SIZE,
                      "until" : until,
                      "commitment" : "confirmed"
                    }],
                })))
                .create();

            until = batch.last();
        }
    }

    fn mock_transactions(server: &mut Server, events: &[TestEvent]) {
        for event in events {
            let responce = json!({
               "jsonrpc": "2.0",
               "result": json!(SolanaMock::get_transaction(event)),
               "id": 1
            });

            server
                .mock("POST", "/")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&responce.to_string())
                .match_body(Matcher::PartialJson(json!({
                    "method": "getConfirmedTransaction",
                    "params": [
                        event.signature,
                        "json"
                      ]
                })))
                .create();
        }
    }

    fn mock_accounts(server: &mut Server) {
        let account_keys = vec![REPORTER, CASE, ADDRESS, ASSET];

        for address in account_keys {
            // TODO: encode account data
            let encoded_account = json!([]);

            let responce = json!({
               "jsonrpc": "2.0",
               "result": encoded_account,
               "id": 1
            });

            server
                .mock("POST", "/")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&responce.to_string())
                .match_body(Matcher::PartialJson(json!({
                    "method": "getAccountInfo",
                    "params": [
                        address,
                        {
                        "commitment": "processed",
                        "encoding": "base64+zstd"
                        }]
                })))
                .create();
        }
    }
}

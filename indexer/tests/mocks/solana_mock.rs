use {
    anchor_lang::AccountSerialize,
    hapi_core::{
        client::solana::{test_helpers::create_test_tx, InstructionData},
        HapiCoreNetwork,
    },
    hapi_indexer::{IndexingCursor, PushData, PushEvent, SOLANA_BATCH_SIZE},
    mockito::{Matcher, ServerGuard},
    serde_json::{json, Value},
    solana_account_decoder::{UiAccount, UiAccountEncoding},
    solana_sdk::{account::Account, pubkey::Pubkey},
    solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
    std::str::FromStr,
};

use core::panic;

use super::{RpcMock, TestBatch};

pub const PROGRAM_ID: &str = "39WzZqJgkK2QuQxV9jeguKRgHE65Q3HywqPwBzdrKn2B";
pub const REPORTER: &str = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
pub const CASE: &str = "DTDk9GEQoVibTuHmTfDUwHehkH4WYd5fpawPfayGRVdi";
pub const ADDRESS: &str = "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC";
pub const ASSET: &str = "5f2iaDyv4yzTudiNc1XR2EkEW5NtVbfZpqmjZ3fhFtaX";

pub struct SolanaMock;

impl RpcMock for SolanaMock {
    fn get_contract_address() -> String {
        PROGRAM_ID.to_string()
    }

    fn get_network() -> HapiCoreNetwork {
        HapiCoreNetwork::Solana
    }

    fn initialization_mock(server: &mut ServerGuard) {
        let response = json!({
           "jsonrpc": "2.0",
           "result": { "feature-set": 289113172, "solana-core": "1.16.7" },
           "id": 1
        });

        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method":"getVersion",
            })))
            .create();
    }

    fn fetching_jobs_mock(
        server: &mut ServerGuard,
        batches: &[TestBatch],
        cursor: &IndexingCursor,
    ) {
        let mut before = None;
        let until = match cursor {
            IndexingCursor::None => None,
            IndexingCursor::Transaction(tx) => Some(tx.to_string()),
            IndexingCursor::Block(_) => panic!("Invalid cursor"),
        };

        for batch in batches {
            let signatures: Vec<Value> = batch
                .iter()
                .map(|payload| {
                    json!({
                        "signature": payload.event.tx_hash,
                        "slot": 100,
                    })
                })
                .collect();

            SolanaMock::mock_batches(server, signatures, &before, &until);

            before = batch.last().map(|event| event.event.tx_hash.clone());
        }

        SolanaMock::mock_batches(server, vec![], &before, &until);
    }

    fn processing_jobs_mock(server: &mut ServerGuard, batch: &TestBatch) {
        for event in batch {
            SolanaMock::mock_transaction(server, &event.event);
            SolanaMock::mock_accounts(server, &event.data);
        }
    }
}

impl SolanaMock {
    fn get_transaction(event: &PushEvent) -> EncodedConfirmedTransactionWithStatusMeta {
        // TODO: what about asset?
        let account_keys = vec![
            String::from(PROGRAM_ID),
            String::default(),
            String::default(),
            String::from(REPORTER),
            String::from(CASE),
            String::from(ADDRESS),
        ];

        let event_name = event.name.to_string();

        create_test_tx(
            &vec![(
                event_name.as_str(),
                InstructionData::Raw(String::from("Some data")),
            )],
            event.tx_hash.clone(),
            account_keys,
        )
    }

    fn mock_batches(
        server: &mut ServerGuard,
        signatures: Vec<Value>,
        before: &Option<String>,
        until: &Option<String>,
    ) {
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
                  "before" : before,
                  "commitment" : "confirmed"
                }],
            })))
            .create();
    }

    fn mock_transaction(server: &mut ServerGuard, event: &PushEvent) {
        let response = json!({
           "jsonrpc": "2.0",
           "result": json!(SolanaMock::get_transaction(&event)),
           "id": 1
        });

        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method": "getTransaction",
                "params": [
                    event.tx_hash,
                    "json"
                  ]
            })))
            .create();
    }

    fn get_account_data(payload_data: PushData) -> (Pubkey, Vec<u8>) {
        let mut data = Vec::new();

        let address = match payload_data {
            PushData::Address(address) => {
                hapi_core_solana::Address {
                    version: 1,
                    bump: 255,
                    network: Pubkey::default(),
                    address: encode_address(&address.address),
                    category: address.category.into(),
                    risk_score: address.risk,
                    case_id: address.case_id.as_u128(),
                    reporter_id: address.reporter_id.as_u128(),
                    confirmations: 0,
                }
                .try_serialize(&mut data)
                .expect("Failed to serialize address");

                ADDRESS
            }
            PushData::Asset(asset) => {
                hapi_core_solana::Asset {
                    version: 1,
                    bump: 255,
                    network: Pubkey::default(),
                    // TODO: encode asset id
                    address: encode_address(&asset.address),
                    id: [0u8; 64],
                    category: asset.category.into(),
                    risk_score: asset.risk,
                    case_id: asset.case_id.as_u128(),
                    reporter_id: asset.reporter_id.as_u128(),
                    confirmations: 0,
                }
                .try_serialize(&mut data)
                .expect("Failed to serialize asset");

                ASSET
            }
            PushData::Case(case) => {
                hapi_core_solana::Case {
                    version: 1,
                    bump: 255,
                    network: Pubkey::default(),
                    id: case.id.as_u128(),
                    name: case.name,
                    reporter_id: case.reporter_id.as_u128(),
                    status: case.status.into(),
                    url: case.url,
                }
                .try_serialize(&mut data)
                .expect("Failed to serialize case");

                CASE
            }
            PushData::Reporter(reporter) => {
                hapi_core_solana::Reporter {
                    version: 1,
                    bump: 255,
                    network: Pubkey::default(),
                    id: reporter.id.as_u128(),
                    name: reporter.name,
                    account: Pubkey::from_str(reporter.account.as_str())
                        .expect("Invalid reporter address"),
                    role: reporter.role.into(),
                    status: reporter.status.into(),
                    unlock_timestamp: reporter.unlock_timestamp,
                    url: reporter.url,
                    stake: reporter.stake.into(),
                }
                .try_serialize(&mut data)
                .expect("Failed to serialize reporter");

                REPORTER
            }
        };

        (Pubkey::from_str(address).expect("Invalid address"), data)
    }

    fn mock_accounts(server: &mut ServerGuard, payload_data: &PushData) {
        let (address, data) = SolanaMock::get_account_data((*payload_data).clone());

        let account = Account {
            lamports: 100,
            data,
            owner: Pubkey::from_str(PROGRAM_ID).expect("Invalid program id"),
            executable: false,
            rent_epoch: 123,
        };

        let encoded_account = UiAccount::encode(
            &address,
            &account,
            UiAccountEncoding::Base64Zstd,
            None,
            None,
        );

        let response = json!({
           "jsonrpc": "2.0",
           "result": {
            "context": { "apiVersion": "1.16.17", "slot": 252201350 },
            "value": json!(encoded_account),
           },
           "id": 1
        });

        println!("RESPONCE : {}", response.to_string());

        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method": "getAccountInfo",
                "params": [
                    address.to_string(),
                ]
            })))
            .create();
    }
}

pub fn encode_address(address: &str) -> [u8; 64] {
    let mut res = [0u8; 64];
    let bytes = address.as_bytes();
    res[..bytes.len()].copy_from_slice(bytes);

    res
}

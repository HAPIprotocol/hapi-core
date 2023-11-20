use {
    anchor_lang::AccountSerialize,
    hapi_core::{
        client::solana::{byte_array_from_str, test_helpers::create_test_tx, InstructionData},
        HapiCoreNetwork,
    },
    hapi_indexer::{IndexingCursor, PushData, SOLANA_BATCH_SIZE},
    mockito::{Matcher, Server, ServerGuard},
    serde_json::{json, Value},
    solana_account_decoder::{UiAccount, UiAccountEncoding},
    solana_sdk::{account::Account, pubkey::Pubkey},
    solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
    std::{str::FromStr, time::Duration},
};

use solana_sdk::signature::Signature;

use super::{RpcMock, TestBatch, TestData};

pub const PROGRAM_ID: &str = "39WzZqJgkK2QuQxV9jeguKRgHE65Q3HywqPwBzdrKn2B";
pub const REPORTER: &str = "C7DNJUKfDVpL9ZZqLnVTG1adj4Yu46JgDB6hiTdMEktX";
pub const CASE: &str = "DTDk9GEQoVibTuHmTfDUwHehkH4WYd5fpawPfayGRVdi";
pub const ADDRESS_OR_ASSET: &str = "WN4cDdcxEEzCVyaFEuG4zzJB6QNqrahtfYpSeeecrmC";

pub struct SolanaMock {
    server: ServerGuard,
}

impl RpcMock for SolanaMock {
    fn get_contract_address() -> String {
        PROGRAM_ID.to_string()
    }

    fn get_network() -> HapiCoreNetwork {
        HapiCoreNetwork::Solana
    }

    fn get_hashes() -> [String; 17] {
        // Solana RPC returns transactions in descending order (latest => earliest):
        // ==> First run: 2 batches of 6 transactions each
        //     -> last tx in second batch is the earliest
        // ==> Second run: 1 batch of 5 transactions
        //     -> first tx in this batch is the latest

        let signatures: [String; 17] = (0..17)
            .map(|_| Signature::new_unique().to_string())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Failed to create signatures");

        signatures
    }

    fn initialize() -> Self {
        let mut server = Server::new();

        let response = json!({
           "jsonrpc": "2.0",
           "result": { "feature-set": 289113172, "solana-core": "1.16.7" },
           "id": 1
        });

        // This method is called quite often before other requests in rpc client
        server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method":"getVersion",
            })))
            .create();

        Self { server }
    }

    fn get_mock_url(&self) -> String {
        self.server.url()
    }

    fn get_cursor(batch: &[TestBatch]) -> IndexingCursor {
        batch
            .first()
            .map(|batch| batch.first().expect("Empty batch"))
            .map(|tx| IndexingCursor::Transaction(tx.hash.clone()))
            .unwrap_or(IndexingCursor::None)
    }

    fn fetching_jobs_mock(&mut self, batches: &[TestBatch], cursor: &IndexingCursor) {
        let mut before = None;
        let until = match cursor {
            IndexingCursor::None => None,
            IndexingCursor::Transaction(tx) => Some(tx.to_string()),
            IndexingCursor::Block(_) => panic!("Invalid cursor"),
        };

        // Mocking all transactions in batches
        for batch in batches {
            let signatures: Vec<Value> = batch
                .iter()
                .map(|data| {
                    json!({
                        "signature": data.hash,
                        "slot": 100,
                    })
                })
                .collect();

            self.mock_batches(signatures, &before, &until);

            before = batch.last().map(|data| data.hash.clone());
        }

        // Mocking last call before processing:
        // from the last fetched transaction in indexer iteration to the current cursor
        self.mock_batches(vec![], &before, &until);

        // Mocking last call in indexer iteration:
        // from the latest to the last processed
        self.mock_batches(
            vec![],
            &None,
            &batches
                .first()
                .map(|batch| batch.first().expect("Empty Batch").hash.clone()),
        );
    }

    fn processing_jobs_mock(&mut self, batch: &TestBatch) {
        for event in batch {
            // Mocking transaction request with instruction
            self.mock_transaction(event.name.to_string(), &event.hash);

            // Mocking accounts request from instruction
            self.mock_accounts(event);
        }
    }

    fn entity_getters_mock(&mut self, _data: Vec<PushData>) {
        unimplemented!()
    }

    fn get_fetching_delay_multiplier() -> u32 {
        6
    }

    fn get_fetching_delay() -> Duration {
        Duration::from_millis(100)
    }
}

impl SolanaMock {
    fn get_transaction(name: String, hash: &str) -> EncodedConfirmedTransactionWithStatusMeta {
        // To reduce redundant code asset and address have common pubkey (same index in account list)
        // It is important  to call them in different indexer launches
        let account_keys = vec![
            String::from(PROGRAM_ID),
            String::default(),
            String::from(REPORTER),
            String::from(CASE),
            String::from(ADDRESS_OR_ASSET),
        ];

        create_test_tx(
            &vec![(
                name.as_str(),
                InstructionData::Raw(String::from("Some data")),
            )],
            hash.to_string(),
            account_keys,
        )
    }

    fn mock_batches(
        &mut self,
        signatures: Vec<Value>,
        before: &Option<String>,
        until: &Option<String>,
    ) {
        let response = json!({
            "jsonrpc": "2.0",
            "result": signatures,
            "id": 1
        });

        self.server
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

    fn mock_transaction(&mut self, name: String, hash: &str) {
        let response = json!({
           "jsonrpc": "2.0",
           "result": json!(SolanaMock::get_transaction(name, hash)),
           "id": 1
        });

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method": "getTransaction",
                "params": [
                    hash,
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

                ADDRESS_OR_ASSET
            }
            PushData::Asset(asset) => {
                let mut id = [0_u8; 32];
                byte_array_from_str(&asset.asset_id.to_string(), &mut id)
                    .expect("Failed to parse asset id");

                hapi_core_solana::Asset {
                    version: 1,
                    bump: 255,
                    network: Pubkey::default(),
                    address: encode_address(&asset.address),
                    id,
                    category: asset.category.into(),
                    risk_score: asset.risk,
                    case_id: asset.case_id.as_u128(),
                    reporter_id: asset.reporter_id.as_u128(),
                    confirmations: 0,
                }
                .try_serialize(&mut data)
                .expect("Failed to serialize asset");

                ADDRESS_OR_ASSET
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

    fn mock_accounts(&mut self, test_data: &TestData) {
        if let Some(payload_data) = &test_data.data {
            let (address, data) = SolanaMock::get_account_data(payload_data.clone());

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

            self.server
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
}

pub fn encode_address(address: &str) -> [u8; 64] {
    let mut res = [0u8; 64];
    let bytes = address.as_bytes();
    res[..bytes.len()].copy_from_slice(bytes);

    res
}

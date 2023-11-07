use {
    hapi_core::HapiCoreNetwork,
    hapi_indexer::IndexingCursor,
    mockito::{Matcher, Server, ServerGuard},
    serde_json::{json, Value},
    std::fmt::LowerHex,
};

use std::{collections::HashMap, str::FromStr};

use enum_extract::let_extract;
use ethers::{
    abi::{Token, Tokenizable},
    types::{Address, Bytes, Log, H256, U128, U256},
};
use hapi_core::client::events::EventName;
use hapi_indexer::PushData;

use super::{RpcMock, TestBatch};

pub const CONTRACT_ADDRESS: &str = "0x2947F98C42597966a0ec25e92843c09ac18Fbab7";
pub const ABI: &str = "../evm/artifacts/contracts/HapiCore.sol/HapiCore.json";

pub struct EvmMock {
    server: ServerGuard,
}

impl RpcMock for EvmMock {
    fn get_contract_address() -> String {
        CONTRACT_ADDRESS.to_string()
    }

    fn get_network() -> HapiCoreNetwork {
        HapiCoreNetwork::Ethereum
    }

    fn get_hashes() -> [String; 17] {
        unimplemented!()
    }

    fn initialize() -> Self {
        let server = Server::new();
        Self { server }
    }

    fn get_mock_url(&self) -> String {
        unimplemented!()
    }

    fn get_cursor(_batch: &[TestBatch]) -> IndexingCursor {
        unimplemented!()
    }

    fn fetching_jobs_mock(&mut self, _batches: &[TestBatch], _cursor: &IndexingCursor) {
        unimplemented!();
    }

    fn processing_jobs_mock(&mut self, _batch: &TestBatch) {
        unimplemented!();
    }
}

impl EvmMock {
    fn latest_block_mock(&mut self, number: u64) {
        let response = json!({
           "jsonrpc": "2.0",
           "result": format!("{number:#x}"),
           "id": 1
        });

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method":"eth_blockNumber",
            })))
            .create();
    }

    fn get_logs(batch: &TestBatch) -> Vec<Log> {
        let mut res = vec![];

        let event_signatures = get_events_signatures();

        for event in batch {
            let signature = event_signatures
                .get(&event.name.to_string())
                .expect("Failed to get event signature");

            let mut log = Log {
                address: CONTRACT_ADDRESS.parse().unwrap(),
                topics: vec![signature.clone()],
                data: Bytes::new(),
                block_hash: None,
                block_number: None,
                transaction_hash: None,
                transaction_index: None,
                log_index: None,
                transaction_log_index: None,
                log_type: None,
                removed: None,
            };

            match event.name {
                EventName::Initialize => {
                    let version_token = Token::Uint(U256::from(1));

                    log.data = ethers::abi::encode(&[version_token]).into();
                }
                EventName::SetAuthority => {
                    let address = H256::from(
                        "0x0000000000000000000000000000000000000001"
                            .parse::<Address>()
                            .expect("Invalid address"),
                    );

                    log.topics
                        .append(&mut vec![address.clone(), address.clone()]);
                }
                EventName::UpdateStakeConfiguration => {
                    let token: Address = "0x000000000000000000000000000000000000dead"
                        .parse()
                        .expect("Invalid address");
                    let unlock_duration: U256 = 100.into();
                    let validator_stake: U256 = 100.into();
                    let tracer_stake: U256 = 100.into();
                    let publisher_stake: U256 = 100.into();
                    let authority_stake: U256 = 100.into();

                    log.data = ethers::abi::encode(&[
                        Token::Address(token),
                        Token::Uint(unlock_duration),
                        Token::Uint(validator_stake),
                        Token::Uint(tracer_stake),
                        Token::Uint(publisher_stake),
                        Token::Uint(authority_stake),
                    ])
                    .into();
                }
                EventName::UpdateRewardConfiguration => {
                    let token: Address = "0x000000000000000000000000000000000000dead"
                        .parse()
                        .expect("Invalid address");
                    let address_confirmation_reward: U256 = 100.into();
                    let address_tracer_reward: U256 = 100.into();
                    let asset_confirmation_reward: U256 = 100.into();
                    let asset_tracer_reward: U256 = 100.into();

                    log.data = ethers::abi::encode(&[
                        Token::Address(token),
                        Token::Uint(address_confirmation_reward),
                        Token::Uint(address_tracer_reward),
                        Token::Uint(asset_confirmation_reward),
                        Token::Uint(asset_tracer_reward),
                    ])
                    .into();
                }
                EventName::CreateReporter
                | EventName::UpdateReporter
                | EventName::ActivateReporter
                | EventName::DeactivateReporter
                | EventName::Unstake => {
                    let_extract!(
                        PushData::Reporter(data),
                        event.data.as_ref().expect("Empty data"),
                        panic!("Wrong message encoding")
                    );

                    let id = data.id.as_u128();
                    let id_topic = H256::from_slice(&id.to_be_bytes());
                    log.topics.append(&mut vec![id_topic]);

                    let reporter: Address = data.account.parse().expect("Invalid address");
                    let role = data.role.clone() as u8;

                    log.data = ethers::abi::encode(&[
                        Token::Address(reporter),
                        Token::Uint(U256::from(role)),
                    ])
                    .into();
                }
                EventName::CreateCase | EventName::UpdateCase => {
                    // TODO: case update - status closed
                }
                EventName::CreateAddress | EventName::UpdateAddress | EventName::ConfirmAddress => {
                    let_extract!(
                        PushData::Address(data),
                        event.data.as_ref().expect("Empty data"),
                        panic!("Wrong message encoding")
                    );

                    let addr: Address = data.address.parse().expect("Invalid address");
                    let risk = data.risk;
                    let category = data.category.clone() as u8;

                    let addr_topic = H256::from(addr);
                    log.topics.append(&mut vec![addr_topic]);

                    log.data = ethers::abi::encode(&[
                        Token::Uint(U256::from(risk)),
                        Token::Uint(U256::from(category)),
                    ])
                    .into();
                }
                EventName::CreateAsset | EventName::UpdateAsset | EventName::ConfirmAsset => {
                    let_extract!(
                        PushData::Asset(data),
                        event.data.as_ref().expect("Empty data"),
                        panic!("Wrong message encoding")
                    );

                    let addr: Address = data.address.parse().expect("Invalid address");
                    let asset_id: U256 = U256::from_str(&data.asset_id.to_string())
                        .expect("Failed to parse asset id");
                    let risk = data.risk;
                    let category = data.category.clone() as u8;

                    let addr_topic = H256::from(addr);
                    log.topics.append(&mut vec![addr_topic]);

                    log.data = ethers::abi::encode(&[
                        Token::Uint(asset_id),
                        Token::Uint(U256::from(risk)),
                        Token::Uint(U256::from(category)),
                    ])
                    .into();
                }
            }

            res.push(log);
        }

        // for event in batch {
        //     let log = Log {
        //         address: CONTRACT_ADDRESS.parse().unwrap(),
        //         topics: vec![],
        //         data: event.data.clone().unwrap().into(),
        //         block_hash: Some(
        //             H256::from_str(
        //                 "0x8243343df08b9751f5ca0c5f8c9c0460d8a9b6351066fae0acbd4d3e776de8bb",
        //             )
        //             .expect("Failed to parse block hash"),
        //         ),
        //         block_number: None,
        //         transaction_hash: None,
        //         transaction_index: None,
        //         log_index: None,
        //         transaction_log_index: None,
        //         log_type: None,
        //         removed: None,
        //     };

        //     res.push(log);
        // }

        res
    }

    fn fetch_logs_mock(&mut self, from_block: u64, batches: &[TestBatch]) {
        let mut from_block = from_block;

        for batch in batches {
            let to_block = from_block + batch.len() as u64;
            let logs = Self::get_logs(batch);

            let response = json!({
               "jsonrpc": "2.0",
               "result": [logs],
               "id": 1
            });

            let params = json!({
              "fromBlock": format!("{from_block:#x}"),
              "toBlock": format!("{to_block:#x}"),
              "address": CONTRACT_ADDRESS,
            });

            self.server
                .mock("POST", "/")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&response.to_string())
                .match_body(Matcher::PartialJson(json!({
                    "method": "eth_getLogs",
                    "params": [ params ]
                })))
                .create();

            from_block = to_block;
        }
    }
}

use ethers::abi::{Abi, Event};
use ethers::utils::keccak256;
use std::fs;

// #[test]
// fn get_events_signatures() {
fn get_events_signatures() -> HashMap<String, H256> {
    let parsed_json: Value =
        serde_json::from_str(&fs::read_to_string(ABI).expect("Failed to read ABI file"))
            .expect("Failed to psarse ABI JSON");

    let abi_entries = parsed_json["abi"]
        .as_array()
        .expect("Failed to find 'abi' key in JSON");

    // Parse the actual ABI.
    let abi: Abi = serde_json::from_value(Value::Array(abi_entries.clone()))
        .expect("Failed to parse ABI JSON");

    let mut signatures = HashMap::new();

    // Extract the event signatures.
    for event in abi.events.values().flatten() {
        let signature = get_signature(&event);
        let topic_hash: H256 = keccak256(signature.as_bytes()).into();

        println!(
            "Event name: {}, Signature Topic: 0x{}",
            event.name,
            topic_hash.to_string()
        );

        signatures.insert(
            EventName::from_str(&event.name)
                .expect("Unknown event name")
                .to_string(),
            topic_hash,
        );
    }

    signatures
}

fn get_signature(event: &Event) -> String {
    let inputs = event
        .inputs
        .iter()
        .map(|param| {
            format!(
                "{}{}",
                param.kind,
                if param.indexed { " indexed" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!("{}({})", event.name, inputs)
}

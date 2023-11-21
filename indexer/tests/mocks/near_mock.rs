use {
    mockito::{Matcher, Server, ServerGuard},
    near_jsonrpc_client::methods::{self, RpcMethod},
    near_jsonrpc_primitives::types::{
        blocks::RpcBlockResponse,
        query::{QueryResponseKind, RpcQueryRequest},
        receipts::{ReceiptReference, RpcReceiptRequest, RpcReceiptResponse},
    },
    near_primitives::{
        hash::CryptoHash,
        types::{AccountId, Balance, BlockReference, Finality, FunctionArgs, Gas, StoreKey},
        views::{BlockHeaderView, CallResult},
    },
    serde_json::{json, Value},
    std::str::FromStr,
};

use {
    hapi_core::{
        client::{
            entities::{address::Address, asset::Asset, case::Case, reporter::Reporter},
            events::EventName,
        },
        HapiCoreNetwork,
    },
    hapi_core_near::{
        AddressView as NearAddress, AssetView as NearAsset, Case as NearCase,
        Reporter as NearReporter,
    },
    hapi_indexer::{IndexingCursor, PushData},
};

use super::{RpcMock, TestBatch, TestData};

pub struct NearMock {
    server: ServerGuard,
}

pub const CONTRACT_ACCOUNT_ID: &str = "hapi.test.near";
pub const REPORTER_ACCOUNT_ID: &str = "reporter.test.near";
pub const PUBLIC_KEY: &str = "ed25519:J4RNLtJ74HayTR7R5Ae6f2DqpY5hwVUAJeB7AkkRqRcJ";

fn contract_id() -> AccountId {
    AccountId::try_from(CONTRACT_ACCOUNT_ID.to_string()).unwrap()
}

fn reporter_id() -> AccountId {
    AccountId::try_from(REPORTER_ACCOUNT_ID.to_string()).unwrap()
}

impl RpcMock for NearMock {
    const STATE_FILE: &'static str = "data/near_state.json";

    fn get_contract_address() -> String {
        CONTRACT_ACCOUNT_ID.to_string()
    }

    fn get_network() -> HapiCoreNetwork {
        HapiCoreNetwork::Near
    }

    fn get_hashes() -> [String; 17] {
        (0..17)
            .map(|i| CryptoHash::hash_bytes(i.to_string().as_bytes()).to_string())
            .collect::<Vec<String>>()
            .try_into()
            .expect("Failed to convert")
    }

    fn generate_address() -> String {
        unimplemented!()
    }

    fn initialize() -> Self {
        let server = Server::new();

        Self { server }
    }

    fn get_mock_url(&self) -> String {
        self.server.url()
    }

    fn get_cursor(batch: &[TestBatch]) -> IndexingCursor {
        batch
            .last()
            .map(|batch| batch.last().expect("Empty batch"))
            .map(|data| IndexingCursor::Block(data.block))
            .unwrap_or(IndexingCursor::None)
    }

    fn fetching_jobs_mock(&mut self, batches: &[TestBatch], _cursor: &IndexingCursor) {
        for batch in batches {
            for data in batch {
                let result =
                    near_jsonrpc_primitives::types::changes::RpcStateChangesInBlockResponse {
                        block_hash: CryptoHash::default(),
                        changes: vec![near_primitives::views::StateChangeWithCauseView {
                            cause:
                                near_primitives::views::StateChangeCauseView::ReceiptProcessing {
                                    receipt_hash: CryptoHash::from_str(data.hash.as_str()).unwrap(),
                                },
                            value: near_primitives::views::StateChangeValueView::DataUpdate {
                                account_id: contract_id(),
                                key: vec![1].into(),
                                value: vec![1].into(),
                            },
                        }],
                    };

                let response = json!({
                    "jsonrpc": "2.0",
                    "result": result,
                    "id": 1
                });

                let payload = methods::EXPERIMENTAL_changes::RpcStateChangesInBlockByTypeRequest {
                    block_reference: BlockReference::BlockId(
                        near_primitives::types::BlockId::Height(data.block),
                    ),
                    state_changes_request:
                        near_primitives::views::StateChangesRequestView::DataChanges {
                            account_ids: vec![contract_id()],
                            key_prefix: StoreKey::from(vec![]),
                        },
                };

                self.server
                    .mock("POST", "/")
                    .with_status(200)
                    .with_header("content-type", "application/json")
                    .with_body(&response.to_string())
                    .match_body(Matcher::PartialJson(get_value_from_method(payload)))
                    .create();

                self.mock_block(data.block);
            }
        }

        if let Some(batch) = batches.last() {
            let response = json!({
                "jsonrpc": "2.0",
                "result": get_default_rpc_block_response(batch.last().unwrap().block),
                "id": 1
            });

            let payload = methods::block::RpcBlockRequest {
                block_reference: BlockReference::Finality(Finality::Final),
            };

            self.server
                .mock("POST", "/")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(&response.to_string())
                .match_body(Matcher::PartialJson(get_value_from_method(payload)))
                .create();
        }
    }

    fn processing_jobs_mock(&mut self, batch: &TestBatch) {
        for data in batch {
            self.mock_transaction(data);
        }
    }

    fn entity_getters_mock(&mut self, data: Vec<PushData>) {
        self.mock_client_get_requests(&data[0], "get_reporter");
        self.mock_client_get_requests(&data[0], "get_reporter_by_account");
        self.mock_client_get_requests(&data[1], "get_case");
        self.mock_client_get_requests(&data[2], "get_address");
        self.mock_client_get_requests(&data[3], "get_asset");
    }

    fn get_fetching_delay_multiplier() -> u32 {
        14
    }
}

impl NearMock {
    fn mock_block(&mut self, block: u64) {
        let response = json!({
            "jsonrpc": "2.0",
            "result":  get_default_rpc_block_response(block),
            "id": 1
        });

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(
                json!({"jsonrpc":"2.0","method":"block","params": { "block_id": block}}
                ),
            ))
            .create();
    }

    fn mock_transaction(&mut self, data: &TestData) {
        let result = make_receipt_response(data);

        let response = json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": 1
        });

        let payload = RpcReceiptRequest {
            receipt_reference: ReceiptReference {
                receipt_id: CryptoHash::from_str(data.hash.as_str()).unwrap(),
            },
        };

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(get_value_from_method(payload)))
            .create();
    }

    fn mock_client_get_requests(&mut self, data: &PushData, method: &str) {
        let args = match data {
            PushData::Reporter(reporter) => {
                if method.eq("get_reporter_by_account") {
                    args_from_json(json!(
                        { "account_id": reporter.account}
                    ))
                } else {
                    args_from_json(json!({ "id": reporter.id.as_u128().to_string()}))
                }
            }
            PushData::Address(address) => args_from_json(json!({ "address": address.address})),
            PushData::Case(case) => args_from_json(json!({ "id": case.id.as_u128().to_string()})),
            PushData::Asset(asset) => {
                args_from_json(json!({ "address": asset.address, "id": asset.asset_id.to_string()}))
            }
        };

        let encoded_entity: Vec<u8> = match data {
            PushData::Reporter(r) => {
                let reporter: NearReporter = r.clone().try_into().expect("Failed to convert");
                serde_json::to_string(&reporter).unwrap().into_bytes()
            }
            PushData::Address(a) => {
                let address: NearAddress = a.clone().try_into().expect("Failed to convert");
                serde_json::to_string(&address).unwrap().into_bytes()
            }
            PushData::Case(c) => {
                let case: NearCase = c.clone().try_into().expect("Failed to convert");
                serde_json::to_string(&case).unwrap().into_bytes()
            }
            PushData::Asset(a) => {
                let asset: NearAsset = a.clone().try_into().expect("Failed to convert");
                serde_json::to_string(&asset).unwrap().into_bytes()
            }
        };

        let result = methods::query::RpcQueryResponse {
            kind: QueryResponseKind::CallResult(CallResult {
                result: encoded_entity,
                logs: vec![],
            }),
            block_height: near_primitives::types::BlockHeight::default(),
            block_hash: CryptoHash::default(),
        };

        let response = json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": 1
        });

        let payload = RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: near_primitives::views::QueryRequest::CallFunction {
                account_id: contract_id(),
                method_name: method.to_string(),
                args: args.clone(),
            },
        };

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(get_value_from_method(payload)))
            .create();
    }
}

fn get_value_from_method<M: RpcMethod>(method: M) -> Value {
    let mut request = methods::to_json(&method).expect("Failed to convert method");
    // delete id field from request because it generates randomly and we can't predict it
    request.as_object_mut().unwrap().remove("id");
    request
}

fn make_receipt_response(data: &TestData) -> RpcReceiptResponse {
    let mut method = data.name.to_string();

    let mut args = json!({"some": "data"});
    if let Some(entity) = &data.data {
        args = match entity {
            PushData::Reporter(reporter) => {
                if data.name == EventName::ActivateReporter {
                    method = "ft_on_transfer".to_string();
                    json!({
                      "sender_id": reporter.account,
                      "amount": reporter.stake.to_string(),
                      "msg": ""
                    })
                } else {
                    get_reporter_json(reporter)
                }
            }
            PushData::Address(address) => get_address_json(address),
            PushData::Case(case) => get_case_json(case),
            PushData::Asset(asset) => get_asset_json(asset),
        };
    };

    let encoded_args = FunctionArgs::from(args.to_string().into_bytes());

    RpcReceiptResponse {
        receipt_view: near_primitives::views::ReceiptView {
            predecessor_id: reporter_id(),
            receiver_id: contract_id(),
            receipt_id: CryptoHash::from_str(data.hash.as_str()).unwrap(),

            receipt: near_primitives::views::ReceiptEnumView::Action {
                signer_id: reporter_id(),
                signer_public_key: near_crypto::PublicKey::from_str(PUBLIC_KEY).unwrap(),
                gas_price: 0,
                output_data_receivers: vec![],
                input_data_ids: vec![],
                actions: vec![near_primitives::views::ActionView::FunctionCall {
                    method_name: method,
                    args: encoded_args,
                    gas: Gas::default(),
                    deposit: Balance::default(),
                }],
            },
        },
    }
}

fn get_reporter_json(data: &Reporter) -> Value {
    json!({
        "id": data.id.as_u128().to_string(),
        "account_id": data.account,
        "role": data.role.to_string(),
        "name": data.name,
        "url": data.url,
    })
}

fn get_address_json(data: &Address) -> Value {
    json!({
        "address": data.address,
        "case_id": data.case_id.as_u128().to_string(),
        "risk_score": data.risk,
        "category": data.category.to_string(),
    })
}

fn get_case_json(data: &Case) -> Value {
    json!({
        "id": data.id.as_u128().to_string(),
        "name": data.name,
        "url": data.url,
    })
}

fn get_asset_json(data: &Asset) -> Value {
    json!({
        "address": data.address,
        "id": data.asset_id.to_string(),
        "case_id": data.case_id.as_u128().to_string(),
        "risk_score": data.risk,
        "category": data.category.to_string(),
    })
}

fn args_from_json(json: Value) -> FunctionArgs {
    FunctionArgs::from(json.to_string().into_bytes())
}

fn get_default_rpc_block_response(block: u64) -> RpcBlockResponse {
    RpcBlockResponse {
        block_view: near_primitives::views::BlockView {
            author: reporter_id(),
            header: BlockHeaderView {
                height: block,
                prev_height: None,
                epoch_id: CryptoHash::default(),
                next_epoch_id: CryptoHash::default(),
                hash: CryptoHash::default(),
                prev_hash: CryptoHash::default(),
                prev_state_root: CryptoHash::default(),
                chunk_receipts_root: CryptoHash::default(),
                chunk_headers_root: CryptoHash::default(),
                chunk_tx_root: CryptoHash::default(),
                outcome_root: CryptoHash::default(),
                chunks_included: 0,
                challenges_root: CryptoHash::default(),
                timestamp: 123,
                timestamp_nanosec: 123,
                random_value: CryptoHash::default(),
                validator_proposals: vec![],
                chunk_mask: vec![],
                gas_price: 0,
                block_ordinal: None,
                rent_paid: Balance::default(),
                validator_reward: Balance::default(),
                total_supply: Balance::default(),
                challenges_result: vec![],
                last_final_block: CryptoHash::default(),
                last_ds_final_block: CryptoHash::default(),
                next_bp_hash: CryptoHash::default(),
                block_merkle_root: CryptoHash::default(),
                epoch_sync_data_hash: None,
                approvals: vec![],
                signature: near_crypto::Signature::default(),
                latest_protocol_version: 0,
            },
            chunks: vec![],
        },
    }
}

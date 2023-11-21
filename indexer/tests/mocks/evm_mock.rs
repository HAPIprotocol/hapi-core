use {
    enum_extract::let_extract,
    ethers::{
        abi::Token,
        prelude::{abigen, SignerMiddleware},
        providers::{Http, Provider},
        signers::{LocalWallet, Signer},
        types::{Address, Block, Bytes, Filter, Log, H256, U256},
        utils::keccak256,
    },
    hapi_core::{client::events::EventName, HapiCoreNetwork},
    hapi_indexer::{IndexingCursor, PushData},
    mockito::{Matcher, Server, ServerGuard},
    rand::RngCore,
    serde_json::json,
    std::{str::FromStr, sync::Arc},
};

use super::{RpcMock, TestBatch, PAGE_SIZE};

pub const CONTRACT_ADDRESS: &str = "0x2947F98C42597966a0ec25e92843c09ac18Fbab7";

abigen!(
    HAPI_CORE_CONTRACT,
    "../evm/artifacts/contracts/HapiCore.sol/HapiCore.json"
);

pub struct EvmMock {
    server: ServerGuard,
    contract: HAPI_CORE_CONTRACT<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl RpcMock for EvmMock {
    const STATE_FILE: &'static str = "data/evm_state.json";

    fn get_contract_address() -> String {
        CONTRACT_ADDRESS.to_string()
    }

    fn get_network() -> HapiCoreNetwork {
        HapiCoreNetwork::Ethereum
    }

    fn get_hashes() -> [String; 17] {
        let signatures: [String; 17] = (0..17)
            .map(|_| format!("0x{}", generate_hash()))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Failed to create signatures");

        signatures
    }

    fn generate_address() -> String {
        ethers::utils::to_checksum(&LocalWallet::new(&mut rand::thread_rng()).address(), None)
    }

    fn get_delay_multiplier() -> u32 {
        // Batch amount
        3
    }

    fn initialize() -> Self {
        let server = Server::new();

        let provider =
            Provider::<Http>::try_from(server.url()).expect("Provider intialization failed");
        let wallet = LocalWallet::new(&mut rand::thread_rng());

        let client = SignerMiddleware::new(provider, wallet);
        let contract = HAPI_CORE_CONTRACT::new(
            CONTRACT_ADDRESS
                .parse::<Address>()
                .expect("Failed to parse address"),
            Arc::new(client),
        );

        Self { server, contract }
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

    fn entity_getters_mock(&mut self, data: Vec<PushData>) {
        data.iter().for_each(|data| self.processing_data_mock(data));
    }

    fn fetching_jobs_mock(&mut self, batches: &[TestBatch], cursor: &IndexingCursor) {
        let mut to_block = 0;
        let mut from_block = match &cursor {
            IndexingCursor::None => 0,
            IndexingCursor::Block(block) => *block + 1,
            _ => panic!("Evm network must have a block cursor"),
        };

        for batch in batches {
            to_block = from_block + batch.len() as u64 - 1;

            let logs = self.get_logs(batch);
            self.logs_request_mock(&logs, from_block, to_block);

            from_block = to_block + 1;
            self.latest_block_mock(to_block);
        }
    }

    fn processing_jobs_mock(&mut self, batch: &TestBatch) {
        batch
            .iter()
            .for_each(|event| self.block_request_mock(event.block));
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

    fn get_logs(&self, batch: &TestBatch) -> Vec<Log> {
        let mut res = vec![];
        let address = CONTRACT_ADDRESS
            .parse::<Address>()
            .expect("Failed to parse address");

        for event in batch {
            let signature = self
                .contract
                .abi()
                .events()
                .find(|e| {
                    EventName::from_str(&e.name).unwrap_or(EventName::Initialize) == event.name
                })
                .map(|e| e.signature())
                .expect("Failed to get event signature");

            let mut log = Log {
                address: address.clone(),
                topics: vec![signature.clone()],
                data: Bytes::new(),
                block_hash: Some(H256::from_low_u64_be(event.block.into())),
                block_number: Some(event.block.into()),
                transaction_hash: Some(
                    H256::from_str(&event.hash).expect("Failed to parse transaction hash"),
                ),
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
                    let address = Self::generate_address()
                        .parse::<Address>()
                        .expect("Invalid address");

                    log.data = ethers::abi::encode(&[Token::Address(address)]).into();
                }
                EventName::UpdateStakeConfiguration => {
                    let token = Self::generate_address()
                        .parse::<Address>()
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
                    let token = Self::generate_address()
                        .parse::<Address>()
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

                    let id_topic = u128_to_bytes(data.id.as_u128()).into();
                    let reporter: Address = data.account.parse().expect("Invalid address");
                    let role = data.role.clone() as u8;

                    log.topics.append(&mut vec![id_topic]);
                    log.data = ethers::abi::encode(&[
                        Token::Address(reporter),
                        Token::Uint(U256::from(role)),
                    ])
                    .into();
                }
                EventName::CreateCase | EventName::UpdateCase => {
                    let_extract!(
                        PushData::Case(data),
                        event.data.as_ref().expect("Empty data"),
                        panic!("Wrong message encoding")
                    );

                    let id_topic = u128_to_bytes(data.id.as_u128()).into();

                    log.topics.append(&mut vec![id_topic]);
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
                    let asset_id: U256 = data.asset_id.clone().into();
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

        res
    }

    fn logs_request_mock(&mut self, logs: &[Log], from_block: u64, to_block: u64) {
        let response = json!({
           "jsonrpc": "2.0",
           "result": logs,
           "id": 1
        });

        let params = Filter::default()
            .address(
                CONTRACT_ADDRESS
                    .parse::<Address>()
                    .expect("Failed to parse address"),
            )
            .from_block(from_block)
            .to_block(to_block);

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
    }

    fn block_request_mock(&mut self, num: u64) {
        let mut block: Block<H256> = Block::default();
        block.timestamp = 123.into();

        let response = json!({
           "jsonrpc": "2.0",
           "result": block,
           "id": 1
        });

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method": "eth_getBlockByNumber",
                "params": [ format!("{num:#x}"), false ]
            })))
            .create();
    }

    fn processing_data_mock(&mut self, data: &PushData) {
        let (raw_tx, result) = match data {
            PushData::Address(address) => {
                let addr = address
                    .address
                    .parse::<Address>()
                    .expect("Failed to parse address");

                let case_id = U256::from_big_endian(&u128_to_bytes(address.case_id.as_u128()));
                let reporter_id =
                    U256::from_big_endian(&u128_to_bytes(address.reporter_id.as_u128()));
                let confirmations = U256::from(address.confirmations);
                let risk = U256::from(address.risk);
                let category = U256::from(address.category.clone() as u8);

                let raw_tx = self.contract.get_address(addr).tx;
                let responce = hex::encode(ethers::abi::encode(&[Token::Tuple(vec![
                    Token::Address(addr),
                    Token::Uint(case_id),
                    Token::Uint(reporter_id),
                    Token::Uint(confirmations),
                    Token::Uint(risk),
                    Token::Uint(category),
                ])]));

                (raw_tx, format!("0x{}", responce))
            }
            PushData::Asset(asset) => {
                let addr = asset
                    .address
                    .parse::<Address>()
                    .expect("Failed to parse address");
                let asset_id = U256::from(asset.asset_id.to_owned());
                let case_id = U256::from_big_endian(&u128_to_bytes(asset.case_id.as_u128()));
                let reporter_id =
                    U256::from_big_endian(&u128_to_bytes(asset.reporter_id.as_u128()));
                let confirmations = U256::from(asset.confirmations);
                let risk = U256::from(asset.risk);
                let category = U256::from(asset.category.clone() as u8);

                let raw_tx = self
                    .contract
                    .get_asset(addr, asset.asset_id.clone().into())
                    .tx;
                let responce = hex::encode(ethers::abi::encode(&[Token::Tuple(vec![
                    Token::Address(addr),
                    Token::Uint(asset_id),
                    Token::Uint(case_id),
                    Token::Uint(reporter_id),
                    Token::Uint(confirmations),
                    Token::Uint(risk),
                    Token::Uint(category),
                ])]));

                (raw_tx, format!("0x{}", responce))
            }
            PushData::Case(case) => {
                let id = U256::from_big_endian(&u128_to_bytes(case.id.as_u128()));
                let name = case.name.to_owned();
                let url = case.url.to_owned();
                let reporter_id = U256::from_big_endian(&u128_to_bytes(case.reporter_id.as_u128()));
                let status = U256::from(case.status.clone() as u8);

                let raw_tx = self.contract.get_case(case.id.as_u128()).tx;
                let responce = hex::encode(ethers::abi::encode(&[Token::Tuple(vec![
                    Token::Uint(id),
                    Token::String(name),
                    Token::Uint(reporter_id),
                    Token::Uint(status),
                    Token::String(url),
                ])]));

                (raw_tx, format!("0x{}", responce))
            }
            PushData::Reporter(reporter) => {
                let reporter_id = U256::from_big_endian(&u128_to_bytes(reporter.id.as_u128()));
                let account = reporter
                    .account
                    .parse::<Address>()
                    .expect("Failed to parse address");
                let name = reporter.name.to_owned();
                let url = reporter.url.to_owned();
                let role = U256::from(reporter.role.clone() as u8);
                let status = U256::from(reporter.status.clone() as u8);
                let stake = U256::from(reporter.stake.to_owned());
                let unlock_timestamp = U256::from(reporter.unlock_timestamp);

                let raw_tx = self.contract.get_reporter(reporter.id.as_u128()).tx;
                let responce = hex::encode(ethers::abi::encode(&[Token::Tuple(vec![
                    Token::Uint(reporter_id),
                    Token::Address(account),
                    Token::String(name),
                    Token::String(url),
                    Token::Uint(role),
                    Token::Uint(status),
                    Token::Uint(stake),
                    Token::Uint(unlock_timestamp),
                ])]));

                (raw_tx, format!("0x{}", responce))
            }
        };

        let tx = serde_json::to_value(raw_tx).expect("Failed to serialize raw transaction");

        let response = json!({
           "jsonrpc": "2.0",
           "result": result ,
           "id": 1
        });

        self.server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&response.to_string())
            .match_body(Matcher::PartialJson(json!({
                "method": "eth_call",
                "params": [ tx, "latest" ]
            })))
            .create();
    }
}

fn u128_to_bytes(value: u128) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    buffer[16..].copy_from_slice(&value.to_be_bytes());

    buffer
}

fn generate_hash() -> String {
    let mut rng = rand::thread_rng();
    let mut data = [0u8; 32];
    rng.fill_bytes(&mut data);

    hex::encode(keccak256(data).to_vec())
}

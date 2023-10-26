use {
    hapi_core::{
        client::{
            entities::{
                address::Address,
                asset::Asset,
                case::{Case, CaseStatus},
                category::Category,
                reporter::{Reporter, ReporterRole, ReporterStatus},
            },
            events::EventName,
        },
        HapiCoreNetwork,
    },
    hapi_indexer::{IndexingCursor, PushData},
    uuid::Uuid,
};

pub mod evm_mock;
pub mod near_mock;
pub mod solana_mock;
pub mod webhook_mock;

pub trait RpcMock {
    fn initialize() -> Self;

    fn get_contract_address() -> String;
    fn get_network() -> HapiCoreNetwork;
    fn get_hashes() -> [String; 6];
    fn get_mock_url(&self) -> String;

    fn fetching_jobs_mock(&mut self, batches: &[TestBatch], cursor: &IndexingCursor);
    fn processing_jobs_mock(&mut self, batch: &TestBatch);
}

pub type TestBatch = Vec<TestData>;

#[derive(Debug, Clone)]
pub struct TestData {
    pub hash: String,
    pub name: EventName,
    pub data: Option<PushData>,
}

// TODO: add other transactions (update_configuration etc.)
pub fn create_test_batches<T: RpcMock>() -> Vec<TestBatch> {
    let hashes = T::get_hashes();

    let data = [
        (
            EventName::CreateReporter,
            Some(PushData::Reporter(Reporter {
                id: Uuid::new_v4(),
                account: "9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b".to_string(),
                role: ReporterRole::Publisher,
                status: ReporterStatus::Active,
                name: String::from("Publisher reporter"),
                url: String::from("https://publisher.com"),
                stake: 1234.into(),
                unlock_timestamp: 123,
            })),
        ),
        (EventName::UpdateStakeConfiguration, None),
        (EventName::ConfirmAddress, None),
        (EventName::ConfirmAsset, None),
        (
            EventName::CreateAddress,
            Some(PushData::Address(Address {
                address: "BGCCDDHfysuuVnaNVtEhhqeT4k9Muyem3Kpgq2U1m9HX".to_string(),
                case_id: Uuid::new_v4(),
                reporter_id: Uuid::new_v4(),
                risk: 5,
                category: Category::ATM,
            })),
        ),
        // (
        //     EventName::CreateAsset,
        //     Some(PushData::Asset(Asset {
        //         address: "0x71C7656EC7ab88b098defB751B7401B5f6d8976F".to_string(),
        //         asset_id: "0x06012c8cf97BEaD5deAe237070F9587f8E7A266d".to_string(),
        //         case_id: Uuid::new_v4(),
        //         reporter_id: Uuid::new_v4(),
        //         risk: 7,
        //         category: Category::DeFi,
        //     })),
        // ),
        (
            EventName::CreateCase,
            Some(PushData::Case(Case {
                id: Uuid::new_v4(),
                name: String::from("Case 1"),
                url: String::from("https://case1.com"),
                status: CaseStatus::Open,
                reporter_id: Uuid::new_v4(),
            })),
        ),
    ];

    let batches: TestBatch = hashes
        .iter()
        .zip(data.iter())
        .map(|(hash, (name, data))| TestData {
            hash: hash.clone(),
            name: name.clone(),
            data: data.clone(),
        })
        .collect();

    let first_batch = batches[0..2].to_vec();
    let second_batch = batches[2..5].to_vec();
    let third_batch = batches[5..].to_vec();

    vec![first_batch, second_batch, third_batch]
}

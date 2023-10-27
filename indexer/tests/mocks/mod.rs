use {
    hapi_core::{
        client::{
            entities::{
                address::Address,
                asset::{Asset, AssetId},
                case::{Case, CaseStatus},
                category::Category,
                reporter::{Reporter, ReporterRole, ReporterStatus},
            },
            events::EventName,
        },
        HapiCoreNetwork,
    },
    hapi_indexer::{IndexingCursor, PushData},
    std::str::FromStr,
    uuid::Uuid,
};

pub mod evm_mock;
pub mod near_mock;
pub mod solana_mock;
pub mod webhook_mock;

pub trait RpcMock {
    // Network mock server initialization
    fn initialize() -> Self;

    // Returns Hapi core contarct address
    fn get_contract_address() -> String;

    // Returns Hapi core network
    fn get_network() -> HapiCoreNetwork;

    // Returns network-specific hashes for 17 events
    fn get_hashes() -> [String; 17];

    // Returns the URL of the network mock server
    fn get_mock_url(&self) -> String;

    // Returns the cursor used in network indexing
    fn get_cursor(batch: &[TestBatch]) -> IndexingCursor;

    // Should contain necessary mocks to handle check for updates
    fn fetching_jobs_mock(&mut self, batches: &[TestBatch], cursor: &IndexingCursor);

    // Should contain mocks to handle processing (not mandatory for all networks)
    fn processing_jobs_mock(&mut self, batch: &TestBatch);
}

pub type TestBatch = Vec<TestData>;

#[derive(Debug, Clone)]
pub struct TestData {
    pub hash: String,
    pub name: EventName,
    pub data: Option<PushData>,
}

// Create test batches: 17 events structured into 3 batches:
// 2 batches for the first launch of the indexer 1 batch for the second
pub fn create_test_batches<T: RpcMock>() -> Vec<TestBatch> {
    let hashes = T::get_hashes();

    let reporter = Reporter {
        id: Uuid::new_v4(),
        account: "9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b".to_string(),
        role: ReporterRole::Publisher,
        status: ReporterStatus::Active,
        name: String::from("Publisher reporter"),
        url: String::from("https://publisher.com"),
        stake: 1234.into(),
        unlock_timestamp: 123,
    };

    let case = Case {
        id: Uuid::new_v4(),
        name: String::from("Case 1"),
        url: String::from("https://case1.com"),
        status: CaseStatus::Open,
        reporter_id: Uuid::new_v4(),
    };

    let address = Address {
        address: "BGCCDDHfysuuVnaNVtEhhqeT4k9Muyem3Kpgq2U1m9HX".to_string(),
        case_id: Uuid::new_v4(),
        reporter_id: Uuid::new_v4(),
        risk: 5,
        category: Category::ATM,
    };

    let asset = Asset {
        address: "0x71C7656EC7ab88b098defB751B7401B5f6d8976F".to_string(),
        asset_id: AssetId::from_str("12345678").expect("Failed to parse asset id"),
        case_id: Uuid::new_v4(),
        reporter_id: Uuid::new_v4(),
        risk: 7,
        category: Category::DeFi,
    };

    let data = [
        // ==> First Run
        // First batch
        (EventName::Initialize, None),
        (EventName::SetAuthority, None),
        (
            EventName::CreateReporter,
            Some(PushData::Reporter(reporter.clone())),
        ),
        (EventName::UpdateStakeConfiguration, None),
        (
            EventName::ActivateReporter,
            Some(PushData::Reporter(reporter.clone())),
        ),
        (
            EventName::UpdateReporter,
            Some(PushData::Reporter(reporter.clone())),
        ),
        // Second batch
        (EventName::UpdateRewardConfiguration, None),
        (EventName::CreateCase, Some(PushData::Case(case.clone()))),
        (EventName::UpdateCase, Some(PushData::Case(case.clone()))),
        (
            EventName::CreateAddress,
            Some(PushData::Address(address.clone())),
        ),
        (
            EventName::UpdateAddress,
            Some(PushData::Address(address.clone())),
        ),
        (EventName::ConfirmAddress, None),
        // ==> Second Run
        // First batch
        (EventName::CreateAsset, Some(PushData::Asset(asset.clone()))),
        (EventName::UpdateAsset, Some(PushData::Asset(asset.clone()))),
        (EventName::ConfirmAsset, None),
        (
            EventName::DeactivateReporter,
            Some(PushData::Reporter(reporter.clone())),
        ),
        (
            EventName::Unstake,
            Some(PushData::Reporter(reporter.clone())),
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

    let first_batch = batches[0..6].to_vec();
    let second_batch = batches[6..12].to_vec();
    let third_batch = batches[12..].to_vec();

    vec![first_batch, second_batch, third_batch]
}

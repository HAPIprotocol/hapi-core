use {
    chrono::Utc,
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
    hapi_indexer::{NetworkData, PushData, PushEvent, PushPayload},
    rand::{distributions::Alphanumeric, thread_rng, Rng},
    std::str::FromStr,
    uuid::Uuid,
};

use super::jwt::get_jwt_id;

pub struct TestData<T> {
    pub data: T,
    pub network_id: String,
}

pub(crate) fn get_test_data(
    network: &HapiCoreNetwork,
    chain_id: Option<String>,
) -> Vec<PushPayload> {
    let mut events = vec![];

    let mut reporter_payload = Reporter {
        id: Uuid::new_v4(),
        account: "0x922ffdfcb57de5dd6f641f275e98b684ce5576a3".to_string(),
        role: ReporterRole::Publisher,
        status: ReporterStatus::Inactive,
        name: String::from("Publisher reporter"),
        url: String::from("https://publisher.com"),
        stake: 0.into(),
        unlock_timestamp: 0,
    };

    let mut case_payload = Case {
        id: Uuid::new_v4(),
        name: String::from("Case 1"),
        url: String::from("https://case1.com"),
        status: CaseStatus::Open,
        reporter_id: reporter_payload.id.to_owned(),
    };

    let mut address_payload = Address {
        address: "0x9e833a87087efd527b1a842742eb0f3548cd82ab".to_string(),
        case_id: case_payload.id.to_owned(),
        reporter_id: reporter_payload.id.to_owned(),
        risk: 6,
        category: Category::DeFi,
        confirmations: 0,
    };

    let mut asset_payload = Asset {
        address: "0xe9dbfa9e9d48393d9d22de10051dcbd91267b756".to_string(),
        asset_id: AssetId::from_str("12345678").expect("Failed to parse asset id"),
        case_id: case_payload.id.to_owned(),
        reporter_id: reporter_payload.id.to_owned(),
        risk: 8,
        category: Category::Gambling,
        confirmations: 0,
    };

    let network_data = NetworkData {
        network: network.to_owned(),
        chain_id,
        indexer_id: Uuid::parse_str(&get_jwt_id()).expect("Failed to parse jwt id"),
    };

    // Create events
    events.push(create_payload(
        network_data.clone(),
        EventName::CreateReporter,
        PushData::Reporter(reporter_payload.clone()),
    ));

    events.push(create_payload(
        network_data.clone(),
        EventName::CreateCase,
        PushData::Case(case_payload.clone()),
    ));

    events.push(create_payload(
        network_data.clone(),
        EventName::CreateAddress,
        PushData::Address(address_payload.clone()),
    ));

    events.push(create_payload(
        network_data.clone(),
        EventName::CreateAsset,
        PushData::Asset(asset_payload.clone()),
    ));

    // Update events
    reporter_payload.role = ReporterRole::Authority;
    reporter_payload.name = String::from("Authority reporter");
    reporter_payload.url = String::from("https://authority.com");

    events.push(create_payload(
        network_data.clone(),
        EventName::UpdateReporter,
        PushData::Reporter(reporter_payload.clone()),
    ));

    reporter_payload.status = ReporterStatus::Active;

    events.push(create_payload(
        network_data.clone(),
        EventName::ActivateReporter,
        PushData::Reporter(reporter_payload.clone()),
    ));

    reporter_payload.status = ReporterStatus::Unstaking;
    reporter_payload.stake = 12345.into();
    reporter_payload.unlock_timestamp = 12345;

    events.push(create_payload(
        network_data.clone(),
        EventName::DeactivateReporter,
        PushData::Reporter(reporter_payload.clone()),
    ));

    reporter_payload.status = ReporterStatus::Inactive;
    reporter_payload.stake = 0.into();
    reporter_payload.unlock_timestamp = 0;

    events.push(create_payload(
        network_data.clone(),
        EventName::Unstake,
        PushData::Reporter(reporter_payload.clone()),
    ));

    case_payload.name = String::from("Closed case 1");
    case_payload.url = String::from("https://closed_case1.com");
    case_payload.status = CaseStatus::Closed;

    events.push(create_payload(
        network_data.clone(),
        EventName::UpdateCase,
        PushData::Case(case_payload.clone()),
    ));

    address_payload.risk = 10;
    address_payload.category = Category::Sanctions;
    address_payload.confirmations = 20;

    events.push(create_payload(
        network_data.clone(),
        EventName::UpdateAddress,
        PushData::Address(address_payload.clone()),
    ));

    asset_payload.risk = 9;
    asset_payload.category = Category::Scam;
    asset_payload.confirmations = 25;

    events.push(create_payload(
        network_data.clone(),
        EventName::UpdateAsset,
        PushData::Asset(asset_payload.clone()),
    ));

    events
}

pub fn create_reporter_data(network: &HapiCoreNetwork, chain_id: Option<String>) -> PushPayload {
    let payload = Reporter {
        id: Uuid::new_v4(),
        account: generate_random_string(),
        role: ReporterRole::Publisher,
        status: ReporterStatus::Inactive,
        name: String::from("Publisher reporter"),
        url: String::from("https://publisher.com"),
        stake: 123.into(),
        unlock_timestamp: 0,
    };

    let network_data = NetworkData {
        network: network.to_owned(),
        chain_id,
        indexer_id: Uuid::parse_str(&get_jwt_id()).expect("Failed to parse jwt id"),
    };

    create_payload(
        network_data,
        EventName::CreateReporter,
        PushData::Reporter(payload.clone()),
    )
}

pub fn create_address_data(
    reporter_id: Uuid,
    case_id: Uuid,
    network: &HapiCoreNetwork,
    chain_id: Option<String>,
) -> PushPayload {
    let payload = Address {
        address: generate_random_string(),
        case_id,
        reporter_id,
        risk: 6,
        category: Category::DeFi,
        confirmations: 0,
    };

    let network_data = NetworkData {
        network: network.to_owned(),
        chain_id,
        indexer_id: Uuid::parse_str(&get_jwt_id()).expect("Failed to parse jwt id"),
    };

    create_payload(
        network_data,
        EventName::CreateAddress,
        PushData::Address(payload.clone()),
    )
}

pub fn create_asset_data(
    reporter_id: Uuid,
    case_id: Uuid,
    network: &HapiCoreNetwork,
    chain_id: Option<String>,
) -> PushPayload {
    let payload = Asset {
        address: generate_random_string(),
        asset_id: AssetId::from_str("12345678").expect("Failed to parse asset id"),
        case_id,
        reporter_id,
        risk: 6,
        category: Category::DeFi,
        confirmations: 0,
    };

    let network_data = NetworkData {
        network: network.to_owned(),
        chain_id,
        indexer_id: Uuid::parse_str(&get_jwt_id()).expect("Failed to parse jwt id"),
    };

    create_payload(
        network_data,
        EventName::CreateAsset,
        PushData::Asset(payload.clone()),
    )
}

fn create_payload(network_data: NetworkData, name: EventName, data: PushData) -> PushPayload {
    let tx_hash = generate_random_string();

    let event = PushEvent {
        name,
        tx_hash,
        tx_index: 0,
        timestamp: Utc::now().timestamp() as u64,
    };

    PushPayload {
        network_data,
        event,
        data,
    }
}

fn generate_random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

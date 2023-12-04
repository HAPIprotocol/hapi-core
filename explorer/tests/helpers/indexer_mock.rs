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
    hapi_indexer::{PushData, PushEvent, PushPayload},
    reqwest::Client,
    std::str::FromStr,
    uuid::Uuid,
};

pub struct IndexerMock {
    pub web_client: Client,
    webhook_url: String,
}

impl IndexerMock {
    pub(crate) fn new(server_addr: &str) -> Self {
        Self {
            web_client: Client::new(),
            webhook_url: format!("{}/{}", server_addr, "events"),
        }
    }
    pub(crate) async fn send_webhook(&self, payload: &PushPayload) {
        let response = self
            .web_client
            .post(&self.webhook_url)
            .json(payload)
            .send()
            .await
            .expect("Failed to send request");

        assert!(response.status().is_success());
    }
}

pub(crate) fn get_test_data(network: &HapiCoreNetwork) -> Vec<PushPayload> {
    let mut events = vec![];

    let mut default_event = PushEvent {
        name: EventName::Initialize,
        tx_hash: "".to_string(),
        tx_index: 0,
        timestamp: 12345,
    };

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
        reporter_id: Uuid::new_v4(),
    };

    let mut address_payload = Address {
        address: "0x9e833a87087efd527b1a842742eb0f3548cd82ab".to_string(),
        case_id: Uuid::new_v4(),
        reporter_id: Uuid::new_v4(),
        risk: 6,
        category: Category::DeFi,
        confirmations: 0,
    };

    let mut asset_payload = Asset {
        address: "0xe9dbfa9e9d48393d9d22de10051dcbd91267b756".to_string(),
        asset_id: AssetId::from_str("12345678").expect("Failed to parse asset id"),
        case_id: Uuid::new_v4(),
        reporter_id: Uuid::new_v4(),
        risk: 8,
        category: Category::Gambling,
        confirmations: 0,
    };

    // Create events
    default_event.name = EventName::CreateReporter;
    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Reporter(reporter_payload.clone()),
    });

    default_event.name = EventName::CreateCase;
    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Case(case_payload.clone()),
    });

    default_event.name = EventName::CreateAddress;
    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Address(address_payload.clone()),
    });

    default_event.name = EventName::CreateAsset;
    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Asset(asset_payload.clone()),
    });

    // Update events
    default_event.name = EventName::UpdateReporter;
    reporter_payload.role = ReporterRole::Authority;
    reporter_payload.name = String::from("Authority reporter");
    reporter_payload.url = String::from("https://authority.com");

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Reporter(reporter_payload.clone()),
    });

    default_event.name = EventName::ActivateReporter;
    reporter_payload.status = ReporterStatus::Active;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Reporter(reporter_payload.clone()),
    });

    default_event.name = EventName::DeactivateReporter;
    reporter_payload.status = ReporterStatus::Unstaking;
    reporter_payload.stake = 12345.into();
    reporter_payload.unlock_timestamp = 12345;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Reporter(reporter_payload.clone()),
    });

    default_event.name = EventName::Unstake;
    reporter_payload.status = ReporterStatus::Inactive;
    reporter_payload.stake = 0.into();
    reporter_payload.unlock_timestamp = 0;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Reporter(reporter_payload.clone()),
    });

    default_event.name = EventName::UpdateCase;
    case_payload.name = String::from("Closed case 1");
    case_payload.url = String::from("https://closed_case1.com");
    case_payload.status = CaseStatus::Closed;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Case(case_payload.clone()),
    });

    default_event.name = EventName::UpdateAddress;
    address_payload.risk = 10;
    address_payload.category = Category::Sanctions;
    address_payload.confirmations = 20;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Address(address_payload.clone()),
    });

    default_event.name = EventName::UpdateAsset;
    asset_payload.risk = 9;
    asset_payload.category = Category::Scam;
    asset_payload.confirmations = 25;

    events.push(PushPayload {
        network: network.clone(),
        event: default_event.clone(),
        data: PushData::Asset(asset_payload.clone()),
    });

    events
}

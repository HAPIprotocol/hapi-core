use hapi_core::client::{
    entities::{
        address::Address, asset::Asset, case::Case, category::Category, reporter::Reporter,
    },
    events::EventName,
};
use hapi_indexer::{
    configuration::IndexerConfiguration, observability::setup_tracing, Indexer, IndexingCursor,
    PersistedState, PushData, PushEvent, PushPayload,
};
use mockito::Server;
use std::{env, path::PathBuf, time::Duration};
use tokio::{spawn, time::sleep, try_join};
use uuid::Uuid;

mod mocks;

use mocks::{
    evm_mock::EvmMock, near_mock::NearMock, solana_mock::SolanaMock,
    webhook_mock::WebhookServiceMock, RpcMock, TestBatch,
};

const TRACING_ENV_VAR: &str = "ENABLE_TRACING";

const WAIT_INTERVAL: Duration = Duration::from_millis(100);
const STATE_FILE: &str = "data/state.json";

// TODO: add other transactions (update_configuration etc.)
fn create_test_batches() -> Vec<TestBatch> {
    vec![vec![PushPayload {
        event: PushEvent {
            name: EventName::CreateAddress,
            tx_hash: "acf0734ab380f3964e1f23b1fd4f5a5125250208ec17ff11c9999451c138949f".to_string(),
            tx_index: 0,
            timestamp: 1690888679,
        },
        data: PushData::Address(Address {
            address: "9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b".to_string(),
            case_id: Uuid::new_v4(),
            reporter_id: Uuid::new_v4(),
            risk: 5,
            category: Category::ATM,
        }),
    }]]
}

async fn test_network<T: RpcMock>() {
    if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
        setup_tracing("debug");
    }

    println!("==> Starting test for {} network", T::get_network());

    let test_data = create_test_batches();
    let mut webhook_mock = WebhookServiceMock::new();
    let mut rpc_mock = Server::new();

    let cfg = IndexerConfiguration {
        network: T::get_network(),
        rpc_node_url: rpc_mock.url(),
        webhook_url: webhook_mock.server.url(),
        contract_address: T::get_contract_address(),
        wait_interval_ms: WAIT_INTERVAL,
        state_file: STATE_FILE.to_string(),
    };

    let mut cursor = IndexingCursor::None;

    // TODO: describe test
    for (index, batches) in test_data.chunks(2).enumerate() {
        let mut indexer = Indexer::new(cfg.clone()).expect("Failed to initialize indexer");

        for batch in batches {
            T::fetching_jobs_mock(&mut rpc_mock, batch, &cursor);
            T::processing_jobs_mock(&mut rpc_mock, batch);
            webhook_mock.set_mocks(batch);
        }

        println!(
            "==> Indexer initialized for {} time, batch mocks created",
            index + 1
        );

        let timer = WAIT_INTERVAL.saturating_mul(5);
        let timer_task = spawn(async move { sleep(timer).await });
        let indexer_task = spawn(async move { indexer.run().await });

        println!(
            "==> Starting indexer with timer: {} millis",
            timer.as_millis()
        );

        try_join!(indexer_task, timer_task)
            .unwrap()
            .0
            .expect("Indexing failed");

        println!("==> Indexing iteration finished, checking results");

        webhook_mock.check_mocks();
        // TODO: check persistent state file + fetch cursor from it

        cursor = PersistedState::from_file(&PathBuf::from(STATE_FILE))
            .expect("Failed to get state")
            .cursor;

        println!("==> Successful indexing iteration");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn solana_indexer_test() {
    test_network::<SolanaMock>().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn evm_indexer_test() {
    test_network::<EvmMock>().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn near_indexer_test() {
    test_network::<NearMock>().await;
}

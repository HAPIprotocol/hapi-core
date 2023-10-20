use hapi_core::client::entities::{address::Address, asset::Asset, case::Case, reporter::Reporter};
use hapi_indexer::{configuration::IndexerConfiguration, Indexer};
use mockito::{Server, ServerGuard};
use std::time::Duration;
use tokio::{spawn, time::sleep, try_join};

mod mocks;

use mocks::{
    evm_mock::EvmMock, near_mock::NearMock, solana_mock::SolanaMock,
    webhook_mock::WebhookServiceMock, RpcMock, TestBatch,
};

const WAIT_INTERVAL: Duration = Duration::from_millis(100);

fn create_test_batches() -> Vec<TestBatch> {
    vec![]
}

async fn test_network<T: RpcMock>() {
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
        state_file: String::from("data/state.json"),
    };

    let mut cursor = None;

    // TODO: describe test
    for (index, batches) in test_data.chunks(2).enumerate() {
        let mut indexer = Indexer::new(cfg.clone()).expect("Failed to initialize indexer");

        for batch in batches {
            T::fetching_jobs_mock(&mut rpc_mock, batch, cursor.clone());
            T::processing_jobs_mock(&mut rpc_mock, batch);
            webhook_mock.set_mocks(batch);
        }

        println!("==> Indexer initialized for {} time, mocks created", index);

        let timer = WAIT_INTERVAL.saturating_mul(2);
        let timer_task = spawn(async move { sleep(timer).await });
        let indexer_task = spawn(async move { indexer.run().await });

        println!(
            "==> Starting indexer with timer: {} millis",
            timer.as_millis()
        );

        let _ = try_join!(indexer_task, timer_task).expect("Indexing failed");

        webhook_mock.check_mocks();
        // TODO: check persistent state file + fetch cursor from it

        println!("==> Successful indexing iteration");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn solana_indexer_test() {
    test_network::<SolanaMock>();
}

#[tokio::test(flavor = "multi_thread")]
async fn evm_indexer_test() {
    test_network::<EvmMock>();
}

#[tokio::test(flavor = "multi_thread")]
async fn near_indexer_test() {
    test_network::<NearMock>();
}

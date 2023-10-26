use {
    hapi_indexer::{
        configuration::IndexerConfiguration, observability::setup_tracing, Indexer, IndexingCursor,
        PersistedState, ITERATION_INTERVAL,
    },
    std::{env, path::PathBuf},
    tokio::{spawn, time::sleep},
};

mod mocks;

use mocks::{
    create_test_batches, evm_mock::EvmMock, near_mock::NearMock, solana_mock::SolanaMock,
    webhook_mock::WebhookServiceMock, RpcMock, TestBatch,
};

const TRACING_ENV_VAR: &str = "ENABLE_TRACING";
const STATE_FILE: &str = "data/state.json";

pub struct IndexerTest<T: RpcMock> {
    webhook_mock: WebhookServiceMock,
    rpc_mock: T,
    cursor: IndexingCursor,
}

impl<T: RpcMock> IndexerTest<T> {
    pub fn new() -> Self {
        if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
            setup_tracing("debug");
        }

        if PathBuf::from(STATE_FILE).exists() {
            std::fs::remove_file(STATE_FILE).expect("Failed to remove state file");
        }

        Self {
            webhook_mock: WebhookServiceMock::new(),
            rpc_mock: T::initialize(),
            cursor: IndexingCursor::None,
        }
    }

    fn create_mocks(&mut self, batches: &[TestBatch]) {
        self.rpc_mock.fetching_jobs_mock(batches, &self.cursor);

        for batch in batches {
            self.rpc_mock.processing_jobs_mock(batch);
            self.webhook_mock.set_mocks(batch);
        }

        println!("==> Batch mocks created");
    }

    async fn indexing_iteration(&self) {
        let cfg = IndexerConfiguration {
            network: T::get_network(),
            rpc_node_url: self.rpc_mock.get_mock_url(),
            webhook_url: self.webhook_mock.server.url(),
            contract_address: T::get_contract_address(),
            wait_interval_ms: ITERATION_INTERVAL,
            state_file: STATE_FILE.to_string(),
        };

        let mut indexer = Indexer::new(cfg).expect("Failed to initialize indexer");
        let timer = ITERATION_INTERVAL.saturating_mul(3);

        println!(
            "==> Starting indexer with timer: {} millis",
            timer.as_millis()
        );

        // Does it panic if error occurs?
        let indexer_task = spawn(async move { indexer.run().await });
        sleep(timer).await;

        indexer_task.abort();
    }

    fn check_cursor(&mut self) {
        // TODO: check persistent state file + fetch cursor from it
        self.cursor = PersistedState::from_file(&PathBuf::from(STATE_FILE))
            .expect("Failed to get state")
            .cursor;
    }

    pub async fn run_test(&mut self) {
        println!("==> Starting test for {} network", T::get_network());

        let test_data = create_test_batches::<T>();

        // TODO: describe test
        for (index, batches) in test_data.chunks(2).enumerate() {
            self.create_mocks(batches);

            println!("==> Running indexer for {} time", index + 1);
            self.indexing_iteration().await;

            println!("==> Indexing iteration finished, checking results");
            self.webhook_mock.check_mocks();
            self.check_cursor();

            println!("==> Successful indexing iteration");
        }
    }
}

impl<T: RpcMock> Drop for IndexerTest<T> {
    fn drop(&mut self) {
        std::fs::remove_file(STATE_FILE).expect("Failed to remove state file");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn solana_indexer_test() {
    IndexerTest::<SolanaMock>::new().run_test().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn evm_indexer_test() {
    IndexerTest::<EvmMock>::new().run_test().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn near_indexer_test() {
    IndexerTest::<NearMock>::new().run_test().await;
}

use {
    hapi_indexer::{
        configuration::IndexerConfiguration, observability::setup_tracing, Indexer, IndexingCursor,
        PersistedState, PushData,
    },
    std::{env, path::PathBuf, time::Duration},
    tokio::time::sleep,
};

mod mocks;

#[cfg(feature = "manual-helper")]
mod simple_listener;

use mocks::{
    create_pushdata, create_test_batches, evm_mock::EvmMock, near_mock::NearMock,
    solana_mock::SolanaMock, webhook_mock::WebhookServiceMock, RpcMock, TestBatch, PAGE_SIZE,
};

const TRACING_ENV_VAR: &str = "ENABLE_TRACING";
const FETCHING_DELAY: Duration = Duration::from_millis(100);

pub struct IndexerTest<T: RpcMock> {
    webhook_mock: WebhookServiceMock,
    rpc_mock: T,
    cursor: IndexingCursor,
}

impl<T: RpcMock> IndexerTest<T> {
    pub fn new() -> Self {
        if env::var(TRACING_ENV_VAR).unwrap_or_default().eq("1") {
            if let Err(e) = setup_tracing("trace") {
                println!("Failed to setup tracing: {}", e);
            }
        }

        drop_state_file(T::STATE_FILE);
        std::env::set_var("INDEXER_PAGE_SIZE", PAGE_SIZE.to_string());

        Self {
            webhook_mock: WebhookServiceMock::new(),
            rpc_mock: T::initialize(),
            cursor: IndexingCursor::None,
        }
    }

    fn create_mocks(&mut self, batches: &[TestBatch], pushdata: Option<Vec<PushData>>) {
        self.rpc_mock.fetching_jobs_mock(batches, &self.cursor);

        if let Some(data) = pushdata {
            self.rpc_mock.entity_getters_mock(data);
        }

        for (index, batch) in batches.iter().enumerate() {
            self.rpc_mock.processing_jobs_mock(batch);
            self.webhook_mock.set_mocks(batch);

            println!("==> Created mocks in {} batch for:", index + 1);
            batch
                .iter()
                .for_each(|event| println!("    --> {:?}", event.name));
        }
    }

    async fn indexing_iteration(&self) -> anyhow::Result<()> {
        let cfg = IndexerConfiguration {
            network: T::get_network(),
            rpc_node_url: self.rpc_mock.get_mock_url(),
            webhook_url: self.webhook_mock.server.url(),
            contract_address: T::get_contract_address(),
            wait_interval_ms: FETCHING_DELAY,
            state_file: T::STATE_FILE.to_string(),
            fetching_delay: FETCHING_DELAY,
            jwt_secret: "my_ultra_secure_secret".to_string(),
        };

        let mut indexer = Indexer::new(cfg).expect("Failed to initialize indexer");
        let indexer_task = async move { indexer.run().await };
        let timer = FETCHING_DELAY.saturating_mul(T::get_delay_multiplier());

        println!(
            "==> Starting indexer with timer: {} millis",
            timer.as_millis()
        );

        tokio::select! {
        Err(e) = indexer_task => {
            println!("==> Indexer task finished before timer, error: {}", e);
            return  Err(e.into());
        }
        _ = sleep(timer) => {
            println!("==> Timer finished, aborting indexer task");
            return Ok(())
        }}
    }

    fn check_cursor(&mut self, batches: &[TestBatch]) {
        self.cursor = PersistedState::from_file(&PathBuf::from(T::STATE_FILE))
            .expect("Failed to get state")
            .cursor;

        assert_eq!(self.cursor, T::get_cursor(batches));
    }

    pub async fn indexing_test(&mut self) {
        println!("\nIndexing test");

        let pushdata = create_pushdata::<T>();
        let test_data = create_test_batches::<T>(&pushdata);

        for (index, batches) in test_data.chunks(2).enumerate() {
            println!("==> Running indexer for {} time", index + 1);

            self.create_mocks(batches, Some(pushdata.clone()));

            self.indexing_iteration().await.unwrap();

            self.webhook_mock.check_mocks();
            self.check_cursor(&batches);

            println!("==> Success: all events were processed, cursor updated\n");
        }
    }

    pub async fn empty_contract_test(&mut self) {
        println!("\nEmpty contract test");

        self.create_mocks(&vec![], None);

        assert!(self.indexing_iteration().await.is_ok());
    }

    pub async fn run_test(&mut self) {
        println!("Starting test for {} network\n", T::get_network());

        // First test: indexer will be running 2 times:
        // 1. First time it will process 2 batches of events
        // 2. Second time it will process 1 batch of events
        // Each time it will check the cursor in
        // persistent state file and received webhook payloads
        self.indexing_test().await;

        // Second test: indexer should stop with log:
        // No valid transactions found on the contract address
        self.empty_contract_test().await;

        println!("Successful indexing on {} network!", T::get_network());
    }
}

impl<T: RpcMock> Drop for IndexerTest<T> {
    fn drop(&mut self) {
        drop_state_file(T::STATE_FILE);
    }
}

fn drop_state_file(file: &'static str) {
    if PathBuf::from(file).exists() {
        std::fs::remove_file(file).expect("Failed to remove state file");
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

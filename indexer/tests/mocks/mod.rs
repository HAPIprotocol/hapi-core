use hapi_indexer::IndexingCursor;
use {hapi_core::HapiCoreNetwork, hapi_indexer::PushPayload, mockito::ServerGuard};

pub mod evm_mock;
pub mod near_mock;
pub mod solana_mock;
pub mod webhook_mock;

pub type TestBatch = Vec<PushPayload>;

pub trait RpcMock {
    fn get_contract_address() -> String;
    fn get_network() -> HapiCoreNetwork;
    fn initialization_mock(server: &mut ServerGuard);
    fn fetching_jobs_mock(server: &mut ServerGuard, batches: &[TestBatch], cursor: &IndexingCursor);
    fn processing_jobs_mock(server: &mut ServerGuard, batch: &TestBatch);
}

use {hapi_core::HapiCoreNetwork, hapi_indexer::IndexingCursor};

use super::{RpcMock, TestBatch};

pub struct NearMock;

impl RpcMock for NearMock {
    const STATE_FILE: &'static str = "data/near_state.json";

    fn get_contract_address() -> String {
        unimplemented!();
    }

    fn get_network() -> HapiCoreNetwork {
        unimplemented!();
    }

    fn get_hashes() -> [String; 17] {
        unimplemented!()
    }

    fn generate_address() -> String {
        unimplemented!()
    }

    fn initialize() -> Self {
        unimplemented!()
    }

    fn get_mock_url(&self) -> String {
        unimplemented!()
    }

    fn get_cursor(_batch: &[TestBatch]) -> IndexingCursor {
        unimplemented!()
    }

    fn fetching_jobs_mock(&mut self, _batches: &[TestBatch], _cursor: &IndexingCursor) {
        unimplemented!();
    }

    fn processing_jobs_mock(&mut self, _batch: &TestBatch) {
        unimplemented!();
    }
}

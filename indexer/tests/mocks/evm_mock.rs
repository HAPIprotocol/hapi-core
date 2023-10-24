use {hapi_core::HapiCoreNetwork, hapi_indexer::IndexingCursor, mockito::ServerGuard};

use super::{RpcMock, TestBatch};

pub struct EvmMock;

impl RpcMock for EvmMock {
    fn get_contract_address() -> String {
        unimplemented!();
    }

    fn get_network() -> HapiCoreNetwork {
        unimplemented!();
    }

    fn initialization_mock(_server: &mut ServerGuard) {
        unimplemented!();
    }

    fn fetching_jobs_mock(
        _server: &mut ServerGuard,
        _batches: &[TestBatch],
        _cursor: &IndexingCursor,
    ) {
        unimplemented!();
    }

    fn processing_jobs_mock(_server: &mut ServerGuard, _batch: &TestBatch) {
        unimplemented!();
    }
}

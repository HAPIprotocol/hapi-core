use {hapi_core::HapiCoreNetwork, mockito::ServerGuard};

use super::{RpcMock, TestBatch};

pub struct NearMock;

impl RpcMock for NearMock {
    fn get_contract_address() -> String {
        unimplemented!();
    }

    fn get_network() -> HapiCoreNetwork {
        unimplemented!();
    }

    fn fetching_jobs_mock(
        _server: &mut ServerGuard,
        _batches: &TestBatch,
        _cursor: Option<String>,
    ) {
        unimplemented!();
    }

    fn processing_jobs_mock(_server: &mut ServerGuard, _batches: &TestBatch) {
        unimplemented!();
    }
}

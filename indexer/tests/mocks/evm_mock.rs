use super::{RpcMock, TestEvent};

pub struct EvmMock;

impl RpcMock for EvmMock {
    fn fetching_jobs_mock(server: &mut Server, batches: &[&[&str]]) {}
    fn processing_jobs_mock(server: &mut Server, events: &[TestEvent]) {}
}

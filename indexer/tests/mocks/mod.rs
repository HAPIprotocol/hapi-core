use mockito::Server;

pub mod solana_mock;

pub struct TestEvent {
    signature: String,
    events: Vec<String>,
}
pub trait RpcMock {
    fn fetching_jobs_mock(server: &mut Server, batches: &[&[&str]]);
    fn processing_jobs_mock(server: &mut Server, events: &[TestEvent]);
}

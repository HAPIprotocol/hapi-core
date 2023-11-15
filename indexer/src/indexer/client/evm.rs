use {anyhow::Result, hapi_core::HapiCoreEvm};

use crate::indexer::push::PushPayload;

use super::indexer_client::FetchingArtifacts;

pub const EVM_PAGE_SIZE: u64 = 100;

pub(super) async fn fetch_evm_jobs(
    _client: &HapiCoreEvm,
    _current_cursor: Option<u64>,
) -> Result<FetchingArtifacts> {
    unimplemented!()
}

pub(super) async fn process_evm_job(
    _client: &HapiCoreEvm,
    _log: &ethers::types::Log,
) -> Result<Option<Vec<PushPayload>>> {
    unimplemented!()
}

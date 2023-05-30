use async_trait::async_trait;

use crate::{
    hapi_core::{
        address::{Address, CreateAddressInput, UpdateAddressInput},
        asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
        case::{Case, CreateCaseInput, UpdateCaseInput},
        configuration::{RewardConfiguration, StakeConfiguration},
        reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        result::{Result, Tx},
    },
    HapiCore,
};

pub struct HapiCoreSolana {}

impl HapiCoreSolana {
    pub async fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HapiCore for HapiCoreSolana {
    async fn set_authority(&self, _address: String) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_authority(&self) -> Result<String> {
        unimplemented!()
    }

    async fn update_stake_configuration(&self, _configuration: StakeConfiguration) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        unimplemented!()
    }

    async fn update_reward_configuration(&self, _configuration: RewardConfiguration) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        unimplemented!()
    }

    async fn create_reporter(&self, _input: CreateReporterInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn update_reporter(&self, _input: UpdateReporterInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_reporter(&self, _id: String) -> Result<Reporter> {
        unimplemented!()
    }
    async fn get_reporter_count(&self) -> Result<u64> {
        unimplemented!()
    }
    async fn get_reporters(&self, _skip: u64, _take: u64) -> Result<Vec<Reporter>> {
        unimplemented!()
    }

    async fn activate_reporter(&self) -> Result<Tx> {
        unimplemented!()
    }
    async fn deactivate_reporter(&self) -> Result<Tx> {
        unimplemented!()
    }
    async fn unstake_reporter(&self) -> Result<Tx> {
        unimplemented!()
    }

    async fn create_case(&self, _input: CreateCaseInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn update_case(&self, _input: UpdateCaseInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_case(&self, _id: String) -> Result<Case> {
        unimplemented!()
    }
    async fn get_case_count(&self) -> Result<u64> {
        unimplemented!()
    }
    async fn get_cases(&self, _skip: u64, _take: u64) -> Result<Vec<Case>> {
        unimplemented!()
    }

    async fn create_address(&self, _input: CreateAddressInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn update_address(&self, _input: UpdateAddressInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_address(&self, _addr: String) -> Result<Address> {
        unimplemented!()
    }
    async fn get_address_count(&self) -> Result<u64> {
        unimplemented!()
    }
    async fn get_addresses(&self, _skip: u64, _take: u64) -> Result<Vec<Address>> {
        unimplemented!()
    }

    async fn create_asset(&self, _input: CreateAssetInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn update_asset(&self, _input: UpdateAssetInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_asset(&self, _address: String, _id: AssetId) -> Result<Asset> {
        unimplemented!()
    }
    async fn get_asset_count(&self) -> Result<u64> {
        unimplemented!()
    }
    async fn get_assets(&self, _skip: u64, _take: u64) -> Result<Vec<Asset>> {
        unimplemented!()
    }
}

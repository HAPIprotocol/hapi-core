use async_trait::async_trait;

use super::{
    address::{Address, CreateAddressInput, UpdateAddressInput},
    asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
    case::{Case, CreateCaseInput, UpdateCaseInput},
    configuration::{RewardConfiguration, StakeConfiguration},
    reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
    result::{Result, Tx},
};

#[async_trait]
pub trait HapiCore {
    fn is_valid_address(&self, address: &str) -> Result<()>;

    async fn set_authority(&self, address: &str) -> Result<Tx>;
    async fn get_authority(&self) -> Result<String>;

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx>;
    async fn get_stake_configuration(&self) -> Result<StakeConfiguration>;

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx>;
    async fn get_reward_configuration(&self) -> Result<RewardConfiguration>;

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx>;
    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx>;
    async fn get_reporter(&self, id: &str) -> Result<Reporter>;
    async fn get_reporter_count(&self) -> Result<u64>;
    async fn get_reporters(&self, skip: u64, take: u64) -> Result<Vec<Reporter>>;

    async fn activate_reporter(&self) -> Result<Tx>;
    async fn deactivate_reporter(&self) -> Result<Tx>;
    async fn unstake_reporter(&self) -> Result<Tx>;

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx>;
    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx>;
    async fn get_case(&self, id: &str) -> Result<Case>;
    async fn get_case_count(&self) -> Result<u64>;
    async fn get_cases(&self, skip: u64, take: u64) -> Result<Vec<Case>>;

    async fn create_address(&self, input: CreateAddressInput) -> Result<Tx>;
    async fn update_address(&self, input: UpdateAddressInput) -> Result<Tx>;
    async fn get_address(&self, addr: &str) -> Result<Address>;
    async fn get_address_count(&self) -> Result<u64>;
    async fn get_addresses(&self, skip: u64, take: u64) -> Result<Vec<Address>>;

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx>;
    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx>;
    async fn get_asset(&self, addr: &str, id: &AssetId) -> Result<Asset>;
    async fn get_asset_count(&self) -> Result<u64>;
    async fn get_assets(&self, skip: u64, take: u64) -> Result<Vec<Asset>>;
}

pub struct HapiCoreOptions {
    pub provider_url: String,
    pub contract_address: String,
    pub private_key: Option<String>,
    pub chain_id: Option<u64>,
}

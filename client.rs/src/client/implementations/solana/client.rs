use async_trait::async_trait;

use crate::{
    client::{
        configuration::{RewardConfiguration, StakeConfiguration},
        entities::{
            address::{Address, CreateAddressInput, UpdateAddressInput},
            asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
            case::{Case, CreateCaseInput, UpdateCaseInput},
            reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        },
        result::{Result, Tx},
        token::TokenContract,
    },
    Amount, HapiCore,
};

pub struct HapiCoreSolana {}

impl HapiCoreSolana {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl HapiCore for HapiCoreSolana {
    fn is_valid_address(&self, _address: &str) -> Result<()> {
        unimplemented!()
    }
    async fn set_authority(&self, _address: &str) -> Result<Tx> {
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
    async fn get_reporter(&self, _id: &str) -> Result<Reporter> {
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
    async fn get_case(&self, _id: &str) -> Result<Case> {
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
    async fn get_address(&self, _addr: &str) -> Result<Address> {
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
    async fn get_asset(&self, _address: &str, _id: &AssetId) -> Result<Asset> {
        unimplemented!()
    }
    async fn get_asset_count(&self) -> Result<u64> {
        unimplemented!()
    }
    async fn get_assets(&self, _skip: u64, _take: u64) -> Result<Vec<Asset>> {
        unimplemented!()
    }
}

pub struct TokenContractSolana {}

impl TokenContractSolana {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl TokenContract for TokenContractSolana {
    fn is_approve_needed(&self) -> bool {
        false
    }

    async fn transfer(&self, _to: &str, _amount: Amount) -> Result<Tx> {
        unimplemented!("`transfer` is not implemented for Near");
    }

    async fn approve(&self, _spender: &str, _amount: Amount) -> Result<Tx> {
        unimplemented!("`approve` is not implemented for Near");
    }

    async fn balance(&self, _addr: &str) -> Result<Amount> {
        unimplemented!("`balance` is not implemented for Near");
    }
}

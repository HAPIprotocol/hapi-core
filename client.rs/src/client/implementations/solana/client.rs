use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster, Program,
};
use solana_cli_config::{Config, CONFIG_FILE};

use async_trait::async_trait;
use std::{str::FromStr, sync::Arc};

use hapi_core_solana::{accounts, instruction, Network};

use crate::{
    client::{
        configuration::{RewardConfiguration, StakeConfiguration},
        entities::{
            address::{Address, CreateAddressInput, UpdateAddressInput},
            asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
            case::{Case, CreateCaseInput, UpdateCaseInput},
            reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        },
        interface::HapiCoreOptions,
        result::{ClientError, Result, Tx},
        token::TokenContract,
    },
    Amount, HapiCore,
};

use super::utils::{get_network_account, get_program_data_account};

pub struct HapiCoreSolana {
    contract: Program<Arc<Keypair>>,
    network: Pubkey,
}

impl HapiCoreSolana {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let program_id = options.contract_address.parse::<Pubkey>().map_err(|e| {
            ClientError::SolanaAddressParseError(format!("`contract-address`: {e}"))
        })?;

        let cluster = Cluster::from_str(&options.provider_url)
            .map_err(|e| ClientError::UrlParseError(format!("`provider-url`: {e}")))?;

        let signer = if let Some(pk) = options.private_key {
            Keypair::from_base58_string(&pk)
        } else {
            let default_config = CONFIG_FILE
                .as_ref()
                .ok_or(ClientError::AbsentDefaultConfig)?;

            let cli_config = Config::load(default_config)
                .map_err(|e| ClientError::UnableToLoadConfig(e.to_string()))?;

            read_keypair_file(cli_config.keypair_path)
                .map_err(|e| ClientError::SolanaKeypairFile(format!("`keypair-path`: {e}")))?
        };

        let client = Client::new(cluster, Arc::new(signer));
        let contract = client.program(program_id)?;

        let network = get_network_account(&options.network.to_string(), &program_id)?;

        Ok(Self { contract, network })
    }
}

#[async_trait(?Send)]
impl HapiCore for HapiCoreSolana {
    fn is_valid_address(&self, address: &str) -> Result<()> {
        address
            .parse::<Pubkey>()
            .map_err(|e| ClientError::SolanaAddressParseError(e.to_string()))?;

        Ok(())
    }

    async fn set_authority(&self, address: &str) -> Result<Tx> {
        let network_data = self.contract.account::<Network>(self.network).await?;

        let new_authority = Pubkey::from_str(address)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`new-authority`: {e}")))?;
        let program_account = self.contract.id();
        let program_data = get_program_data_account(&program_account)?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::SetAuthority {
                authority: network_data.authority,
                network: self.network,
                new_authority,
                program_account,
                program_data,
            })
            .args(instruction::SetAuthority)
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_authority(&self) -> Result<String> {
        let data = self.contract.account::<Network>(self.network).await?;

        Ok(data.authority.to_string())
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let network_data = self.contract.account::<Network>(self.network).await?;

        let stake_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let stake_configuration = hapi_core_solana::StakeConfiguration {
            unlock_duration: configuration.unlock_duration.into(),
            validator_stake: configuration.validator_stake.into(),
            tracer_stake: configuration.tracer_stake.into(),
            publisher_stake: configuration.publisher_stake.into(),
            authority_stake: configuration.authority_stake.into(),
            // Appraiser stake is used only in solana
            appraiser_stake: network_data.stake_configuration.appraiser_stake.into(),
        };

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateStakeConfiguration {
                authority: network_data.authority,
                network: self.network,
                stake_mint,
            })
            .args(instruction::UpdateStakeConfiguration {
                stake_configuration,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        let data = self.contract.account::<Network>(self.network).await?;

        let res = StakeConfiguration {
            token: data.stake_mint.to_string(),
            unlock_duration: data.stake_configuration.unlock_duration,
            validator_stake: data.stake_configuration.validator_stake.into(),
            tracer_stake: data.stake_configuration.tracer_stake.into(),
            publisher_stake: data.stake_configuration.publisher_stake.into(),
            authority_stake: data.stake_configuration.authority_stake.into(),
        };

        Ok(res)
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let network_data = self.contract.account::<Network>(self.network).await?;

        let reward_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let reward_configuration = hapi_core_solana::RewardConfiguration {
            address_confirmation_reward: configuration.address_confirmation_reward.into(),
            address_tracer_reward: configuration.address_tracer_reward.into(),
            asset_confirmation_reward: configuration.asset_confirmation_reward.into(),
            asset_tracer_reward: configuration.asset_tracer_reward.into(),
        };

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateRewardConfiguration {
                authority: network_data.authority,
                network: self.network,
                reward_mint,
            })
            .args(instruction::UpdateRewardConfiguration {
                reward_configuration,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        let data = self.contract.account::<Network>(self.network).await?;

        let res: RewardConfiguration = RewardConfiguration {
            token: data.reward_mint.to_string(),
            address_confirmation_reward: data
                .reward_configuration
                .address_confirmation_reward
                .into(),
            address_tracer_reward: data.reward_configuration.address_tracer_reward.into(),
            asset_confirmation_reward: data.reward_configuration.asset_confirmation_reward.into(),
            asset_tracer_reward: data.reward_configuration.asset_tracer_reward.into(),
        };

        Ok(res)
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

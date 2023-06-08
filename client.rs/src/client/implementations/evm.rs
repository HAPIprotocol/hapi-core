use async_trait::async_trait;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider as EthersProvider},
    signers::LocalWallet,
    types::Address as EthAddress,
};
use ethers_signers::Signer as EthersSigner;
use std::{str::FromStr, sync::Arc};

use crate::{
    client::{
        address::{Address, CreateAddressInput, UpdateAddressInput},
        asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
        case::{Case, CreateCaseInput, UpdateCaseInput},
        configuration::{RewardConfiguration, StakeConfiguration},
        reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        result::{ClientError, Result, Tx},
    },
    HapiCore,
};

pub struct HapiCoreEvmOptions {
    pub provider_url: String,
    pub contract_address: String,
    pub private_key: Option<String>,
}

abigen!(
    CONTRACT,
    "../evm/artifacts/contracts/HapiCore.sol/HapiCore.json"
);

type Provider = EthersProvider<Http>;
type Signer = SignerMiddleware<Provider, LocalWallet>;

pub struct HapiCoreEvm {
    pub provider: Provider,
    pub signer: LocalWallet,
    pub contract: CONTRACT<Signer>,
    pub client: Arc<Signer>,
}

impl HapiCoreEvm {
    pub fn new(options: HapiCoreEvmOptions) -> Result<Self> {
        let provider = Provider::try_from(options.provider_url.as_str())
            .map_err(|e| ClientError::UrlParseError(e.to_string()))?;

        let signer = LocalWallet::from_str(options.private_key.unwrap_or_default().as_str())
            .map_err(|e| ClientError::Ethers(e.to_string()))?
            .with_chain_id(31337_u64);

        let client = Signer::new(provider.clone(), signer.clone());

        let client = Arc::new(client);

        let contract_address: EthAddress =
            options
                .contract_address
                .parse()
                .map_err(|e: <EthAddress as FromStr>::Err| {
                    ClientError::EthAddressParse(e.to_string())
                })?;

        let contract: CONTRACT<Signer> = CONTRACT::new(contract_address, client.clone());

        Ok(Self {
            provider,
            signer,
            contract,
            client,
        })
    }
}

impl From<contract::StakeConfiguration> for StakeConfiguration {
    fn from(config: contract::StakeConfiguration) -> Self {
        StakeConfiguration {
            token: config.token.to_string(),
            unlock_duration: config.unlock_duration.as_u64(),
            validator_stake: config.validator_stake.into(),
            tracer_stake: config.tracer_stake.into(),
            publisher_stake: config.publisher_stake.into(),
            authority_stake: config.authority_stake.into(),
        }
    }
}

impl From<contract::RewardConfiguration> for RewardConfiguration {
    fn from(config: contract::RewardConfiguration) -> Self {
        RewardConfiguration {
            token: config.token.to_string(),
            address_confirmation_reward: config.address_confirmation_reward.into(),
            tracer_reward: config.tracer_reward.into(),
        }
    }
}

#[async_trait]
impl HapiCore for HapiCoreEvm {
    async fn set_authority(&self, address: &str) -> Result<Tx> {
        let authority: EthAddress =
            address
                .parse()
                .map_err(|e: <EthAddress as std::str::FromStr>::Err| {
                    ClientError::EthAddressParse(e.to_string())
                })?;

        self.contract
            .set_authority(authority)
            .send()
            .await
            .map_err(|e| ClientError::Ethers(format!("set_authority failed: {e}")))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "set_authority failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn get_authority(&self) -> Result<String> {
        self.contract
            .authority()
            .call()
            .await
            .map_err(|e| ClientError::Ethers(format!("get_authority failed: {e}")))
            .map(|a| format!("{a:?}"))
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let token =
            configuration
                .token
                .parse()
                .map_err(|e: <EthAddress as std::str::FromStr>::Err| {
                    ClientError::EthAddressParse(e.to_string())
                })?;

        self.contract
            .update_stake_configuration(
                token,
                configuration.unlock_duration.into(),
                configuration.validator_stake.into(),
                configuration.tracer_stake.into(),
                configuration.publisher_stake.into(),
                configuration.authority_stake.into(),
            )
            .send()
            .await
            .map_err(|e| ClientError::Ethers(format!("update_stake_configuration failed: {e}")))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "update_stake_configuration failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }
    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        self.contract
            .stake_configuration()
            .call()
            .await
            .map_err(|e| ClientError::Ethers(format!("get_stake_configuration failed: {e}")))
            .map(|c| c.into())
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let token =
            configuration
                .token
                .parse()
                .map_err(|e: <EthAddress as std::str::FromStr>::Err| {
                    ClientError::EthAddressParse(e.to_string())
                })?;

        self.contract
            .update_reward_configuration(
                token,
                configuration.address_confirmation_reward.into(),
                configuration.tracer_reward.into(),
            )
            .send()
            .await
            .map_err(|e| ClientError::Ethers(format!("update_reward_configuration failed: {e}")))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "update_reward_configuration failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }
    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        self.contract
            .reward_configuration()
            .call()
            .await
            .map_err(|e| ClientError::Ethers(format!("get_reward_configuration failed: {e}")))
            .map(|c| c.into())
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

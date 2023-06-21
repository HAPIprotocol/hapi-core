use async_trait::async_trait;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider as EthersProvider},
    signers::{LocalWallet, Signer as EthersSigner},
    types::Address as EthAddress,
    utils::to_checksum,
};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

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

pub mod error;
pub mod options;
pub mod token;

use error::map_ethers_error;
use options::HapiCoreEvmOptions;

abigen!(
    HAPI_CORE_CONTRACT,
    "../evm/artifacts/contracts/HapiCore.sol/HapiCore.json"
);

pub(super) type Provider = EthersProvider<Http>;
pub(super) type Signer = SignerMiddleware<Provider, LocalWallet>;

pub struct HapiCoreEvm {
    pub provider: Provider,
    pub signer: LocalWallet,
    pub contract: HAPI_CORE_CONTRACT<Signer>,
    pub client: Arc<Signer>,
}

impl HapiCoreEvm {
    pub fn new(options: HapiCoreEvmOptions) -> Result<Self> {
        let provider = Provider::try_from(options.provider_url.as_str())
            .map_err(|e| ClientError::UrlParseError(format!("`provider_url`: {e}")))?;

        let signer = LocalWallet::from_str(options.private_key.unwrap_or_default().as_str())
            .map_err(|e| ClientError::Ethers(format!("`private_key`: {e}")))?
            .with_chain_id(options.chain_id.unwrap_or(31337_u64));

        let client = Signer::new(provider.clone(), signer.clone());

        let client = Arc::new(client);

        let contract_address: EthAddress = options
            .contract_address
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`contract_address``: {e}")))?;

        let contract: HAPI_CORE_CONTRACT<Signer> =
            HAPI_CORE_CONTRACT::new(contract_address, client.clone());

        Ok(Self {
            provider,
            signer,
            contract,
            client,
        })
    }
}

impl From<hapi_core_contract::StakeConfiguration> for StakeConfiguration {
    fn from(config: hapi_core_contract::StakeConfiguration) -> Self {
        StakeConfiguration {
            token: to_checksum(&config.token, None),
            unlock_duration: config.unlock_duration.as_u64(),
            validator_stake: config.validator_stake.into(),
            tracer_stake: config.tracer_stake.into(),
            publisher_stake: config.publisher_stake.into(),
            authority_stake: config.authority_stake.into(),
        }
    }
}

impl From<hapi_core_contract::RewardConfiguration> for RewardConfiguration {
    fn from(config: hapi_core_contract::RewardConfiguration) -> Self {
        RewardConfiguration {
            token: to_checksum(&config.token, None),
            address_confirmation_reward: config.address_confirmation_reward.into(),
            tracer_reward: config.tracer_reward.into(),
        }
    }
}

impl TryFrom<hapi_core_contract::Reporter> for Reporter {
    type Error = ClientError;

    fn try_from(reporter: hapi_core_contract::Reporter) -> Result<Self> {
        Ok(Reporter {
            id: Uuid::from_u128(reporter.id),
            account: to_checksum(&reporter.account, None),
            name: reporter.name.to_string(),
            url: reporter.url.to_string(),
            role: reporter.role.try_into()?,
            status: reporter.status.try_into()?,
            stake: reporter.stake.into(),
            unlock_timestamp: reporter.unlock_timestamp.as_u64(),
        })
    }
}

impl TryFrom<hapi_core_contract::Case> for Case {
    type Error = ClientError;

    fn try_from(case: hapi_core_contract::Case) -> Result<Self> {
        Ok(Case {
            id: Uuid::from_u128(case.id),
            name: case.name.to_string(),
            url: case.url.to_string(),
            status: case.status.try_into()?,
        })
    }
}

impl TryFrom<hapi_core_contract::Address> for Address {
    type Error = ClientError;

    fn try_from(address: hapi_core_contract::Address) -> Result<Self> {
        Ok(Address {
            address: to_checksum(&address.addr, None),
            case_id: Uuid::from_u128(address.case_id),
            reporter_id: Uuid::from_u128(address.reporter_id),
            risk: address.risk,
            category: address.category.try_into()?,
        })
    }
}

impl TryFrom<hapi_core_contract::Asset> for Asset {
    type Error = ClientError;

    fn try_from(asset: hapi_core_contract::Asset) -> Result<Self> {
        Ok(Asset {
            address: to_checksum(&asset.addr, None),
            asset_id: asset.asset_id.into(),
            case_id: Uuid::from_u128(asset.case_id),
            reporter_id: Uuid::from_u128(asset.reporter_id),
            risk: asset.risk,
            category: asset.category.try_into()?,
        })
    }
}

#[async_trait]
impl HapiCore for HapiCoreEvm {
    async fn set_authority(&self, address: &str) -> Result<Tx> {
        let authority: EthAddress = address
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`address`: {e}")))?;

        self.contract
            .set_authority(authority)
            .send()
            .await
            .map_err(|e| map_ethers_error("set_authority", e))?
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
            .map_err(|e| map_ethers_error("get_authority", e))
            .map(|a| format!("{a:?}"))
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let token = configuration
            .token
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`token`: {e}")))?;

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
            .map_err(|e| map_ethers_error("update_stake_configuration", e))?
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
            .map_err(|e| map_ethers_error("get_stake_configuration", e))
            .map(|c| c.into())
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let token = configuration
            .token
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`token`: {e}")))?;

        self.contract
            .update_reward_configuration(
                token,
                configuration.address_confirmation_reward.into(),
                configuration.tracer_reward.into(),
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("update_reward_configuration", e))?
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
            .map_err(|e| map_ethers_error("get_reward_configuration", e))
            .map(|c| c.into())
    }

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx> {
        let addr = input
            .account
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`addr`: {e}")))?;

        self.contract
            .create_reporter(
                input.id.as_u128(),
                addr,
                input.role as u8,
                input.name,
                input.url,
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("create_reporter", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "create_reporter failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx> {
        let addr = input
            .account
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`addr`: {e}")))?;

        self.contract
            .update_reporter(
                input.id.as_u128(),
                addr,
                input.role as u8,
                input.name,
                input.url,
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("update_reporter", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "update_reporter failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let id = id.parse::<Uuid>()?.as_u128();

        self.contract
            .get_reporter(id)
            .call()
            .await
            .map_err(|e| map_ethers_error("get_reporter", e))
            .map(|c| c.try_into())?
    }

    async fn get_reporter_count(&self) -> Result<u64> {
        self.contract
            .get_reporter_count()
            .call()
            .await
            .map_err(|e| map_ethers_error("get_reporter_count", e))
            .map(|c| c.as_u64())
    }

    async fn get_reporters(&self, skip: u64, take: u64) -> Result<Vec<Reporter>> {
        self.contract
            .get_reporters(skip.into(), take.into())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_reporters", e))
            .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }

    async fn activate_reporter(&self) -> Result<Tx> {
        self.contract
            .activate_reporter()
            .send()
            .await
            .map_err(|e| map_ethers_error("activate_reporter", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`activate_reporter` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn deactivate_reporter(&self) -> Result<Tx> {
        self.contract
            .deactivate_reporter()
            .send()
            .await
            .map_err(|e| map_ethers_error("deactivate_reporter", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`deactivate_reporter` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn unstake_reporter(&self) -> Result<Tx> {
        self.contract
            .unstake()
            .send()
            .await
            .map_err(|e| map_ethers_error("unstake_reporter", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`unstake_reporter` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx> {
        self.contract
            .create_case(input.id.as_u128(), input.name, input.url)
            .send()
            .await
            .map_err(|e| map_ethers_error("create_case", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`create_case` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx> {
        self.contract
            .update_case(
                input.id.as_u128(),
                input.name,
                input.url,
                input.status as u8,
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("update_case", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`update_case` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn get_case(&self, id: &str) -> Result<Case> {
        self.contract
            .get_case(id.parse::<Uuid>()?.as_u128())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_case", e))
            .map(|c| c.try_into())?
    }

    async fn get_case_count(&self) -> Result<u64> {
        self.contract
            .get_case_count()
            .call()
            .await
            .map_err(|e| map_ethers_error("get_case_count", e))
            .map(|c| c.as_u64())
    }

    async fn get_cases(&self, skip: u64, take: u64) -> Result<Vec<Case>> {
        self.contract
            .get_cases(skip.into(), take.into())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_cases", e))
            .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }

    async fn create_address(&self, input: CreateAddressInput) -> Result<Tx> {
        let case_id = input.case_id.as_u128();
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        self.contract
            .create_address(address, case_id, input.risk, input.category as u8)
            .send()
            .await
            .map_err(|e| map_ethers_error("create_address", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`create_address` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn update_address(&self, input: UpdateAddressInput) -> Result<Tx> {
        let case_id = input.case_id.as_u128();
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        self.contract
            .update_address(address, input.risk, input.category as u8, case_id)
            .send()
            .await
            .map_err(|e| map_ethers_error("update_address", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`update_address` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn get_address(&self, address: &str) -> Result<Address> {
        let address = address.parse().map_err(|e| {
            ClientError::Ethers(format!("failed to parse address `{}`: {}", address, e))
        })?;

        self.contract
            .get_address(address)
            .call()
            .await
            .map_err(|e| map_ethers_error("get_address", e))
            .map(|a| a.try_into())?
    }

    async fn get_address_count(&self) -> Result<u64> {
        self.contract
            .get_address_count()
            .call()
            .await
            .map_err(|e| map_ethers_error("get_address_count", e))
            .map(|c| c.as_u64())
    }

    async fn get_addresses(&self, skip: u64, take: u64) -> Result<Vec<Address>> {
        self.contract
            .get_addresses(skip.into(), take.into())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_addresses", e))
            .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx> {
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        self.contract
            .create_asset(
                address,
                input.asset_id.into(),
                input.case_id.as_u128(),
                input.risk,
                input.category as u8,
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("create_asset", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`create_asset` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx> {
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        self.contract
            .update_asset(
                address,
                input.asset_id.into(),
                input.risk,
                input.category as u8,
                input.case_id.as_u128(),
            )
            .send()
            .await
            .map_err(|e| map_ethers_error("update_asset", e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`update_asset` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn get_asset(&self, address: &str, id: &AssetId) -> Result<Asset> {
        let address = address.parse().map_err(|e| {
            ClientError::Ethers(format!("failed to parse address `{}`: {}", address, e))
        })?;

        self.contract
            .get_asset(address, id.clone().into())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_asset", e))
            .map(|a| a.try_into())?
    }

    async fn get_asset_count(&self) -> Result<u64> {
        self.contract
            .get_asset_count()
            .call()
            .await
            .map_err(|e| map_ethers_error("get_asset_count", e))
            .map(|c| c.as_u64())
    }

    async fn get_assets(&self, skip: u64, take: u64) -> Result<Vec<Asset>> {
        self.contract
            .get_assets(skip.into(), take.into())
            .call()
            .await
            .map_err(|e| map_ethers_error("get_assets", e))
            .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }
}

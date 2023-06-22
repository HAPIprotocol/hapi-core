use async_trait::async_trait;
use ethers::{
    prelude::{abigen, SignerMiddleware},
    providers::{Http, Provider as EthersProvider},
    signers::{LocalWallet, Signer as EthersSigner},
    types::Address as EthAddress,
};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use crate::{
    client::{
        address::{Address, CreateAddressInput, UpdateAddressInput},
        asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
        case::{Case, CreateCaseInput, UpdateCaseInput},
        configuration::{RewardConfiguration, StakeConfiguration},
        interface::HapiCoreOptions,
        reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        result::{ClientError, Result, Tx},
    },
    HapiCore,
};

mod conversion;
mod error;
pub mod token;

use error::map_ethers_error;

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
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let provider = Provider::try_from(options.provider_url.as_str())
            .map_err(|e| ClientError::UrlParseError(format!("`provider-url`: {e}")))?;

        let signer = LocalWallet::from_str(options.private_key.unwrap_or_default().as_str())
            .map_err(|e| ClientError::Ethers(format!("`private-key`: {e}")))?
            .with_chain_id(options.chain_id.unwrap_or(31337_u64));

        let client = Signer::new(provider.clone(), signer.clone());

        let client = Arc::new(client);

        let contract_address: EthAddress = options
            .contract_address
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`contract-address`: {e}")))?;

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

macro_rules! handle_send {
    ($call:expr, $method_name:expr) => {
        $call
            .send()
            .await
            .map_err(|e| map_ethers_error($method_name, e))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(format!(
                        "`{}` failed: no receipt",
                        $method_name
                    )))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    };
}

macro_rules! handle_call {
    ($call:expr, $method_name:expr) => {
        $call
            .call()
            .await
            .map_err(|e| map_ethers_error($method_name, e))
    };
}

#[async_trait]
impl HapiCore for HapiCoreEvm {
    fn is_valid_address(&self, address: &str) -> Result<()> {
        address
            .parse::<EthAddress>()
            .map_err(|e| ClientError::EthAddressParse(e.to_string()))?;

        Ok(())
    }

    async fn set_authority(&self, address: &str) -> Result<Tx> {
        let authority: EthAddress = address
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`address`: {e}")))?;

        handle_send!(self.contract.set_authority(authority), "set_authority")
    }

    async fn get_authority(&self) -> Result<String> {
        handle_call!(self.contract.authority(), "authority").map(|a| format!("{a:?}"))
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let token = configuration
            .token
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`token`: {e}")))?;

        handle_send!(
            self.contract.update_stake_configuration(
                token,
                configuration.unlock_duration.into(),
                configuration.validator_stake.into(),
                configuration.tracer_stake.into(),
                configuration.publisher_stake.into(),
                configuration.authority_stake.into(),
            ),
            "update_stake_configuration"
        )
    }
    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        handle_call!(self.contract.stake_configuration(), "stake_configuration").map(|c| c.into())
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let token = configuration
            .token
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`token`: {e}")))?;

        handle_send!(
            self.contract.update_reward_configuration(
                token,
                configuration.address_confirmation_reward.into(),
                configuration.tracer_reward.into(),
            ),
            "update_reward_configuration"
        )
    }

    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        handle_call!(self.contract.reward_configuration(), "reward_configuration").map(|c| c.into())
    }

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx> {
        let addr = input
            .account
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`addr`: {e}")))?;

        handle_send!(
            self.contract.create_reporter(
                input.id.as_u128(),
                addr,
                input.role as u8,
                input.name,
                input.url,
            ),
            "create_reporter"
        )
    }

    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx> {
        let addr = input
            .account
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`addr`: {e}")))?;

        handle_send!(
            self.contract.update_reporter(
                input.id.as_u128(),
                addr,
                input.role as u8,
                input.name,
                input.url,
            ),
            "update_reporter"
        )
    }

    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let id = id.parse::<Uuid>()?.as_u128();

        handle_call!(self.contract.get_reporter(id), "get_reporter").map(|c| c.try_into())?
    }

    async fn get_reporter_count(&self) -> Result<u64> {
        handle_call!(self.contract.get_reporter_count(), "get_reporter_count").map(|c| c.as_u64())
    }

    async fn get_reporters(&self, skip: u64, take: u64) -> Result<Vec<Reporter>> {
        handle_call!(
            self.contract.get_reporters(skip.into(), take.into()),
            "get_reporters"
        )
        .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }

    async fn activate_reporter(&self) -> Result<Tx> {
        handle_send!(self.contract.activate_reporter(), "activate_reporter")
    }

    async fn deactivate_reporter(&self) -> Result<Tx> {
        handle_send!(self.contract.deactivate_reporter(), "deactivate_reporter")
    }

    async fn unstake_reporter(&self) -> Result<Tx> {
        handle_send!(self.contract.unstake(), "unstake")
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx> {
        handle_send!(
            self.contract
                .create_case(input.id.as_u128(), input.name, input.url),
            "create_case"
        )
    }

    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx> {
        handle_send!(
            self.contract.update_case(
                input.id.as_u128(),
                input.name,
                input.url,
                input.status as u8,
            ),
            "update_case"
        )
    }

    async fn get_case(&self, id: &str) -> Result<Case> {
        handle_call!(
            self.contract.get_case(id.parse::<Uuid>()?.as_u128()),
            "get_case"
        )
        .map(|c| c.try_into())?
    }

    async fn get_case_count(&self) -> Result<u64> {
        handle_call!(self.contract.get_case_count(), "get_case_count").map(|c| c.as_u64())
    }

    async fn get_cases(&self, skip: u64, take: u64) -> Result<Vec<Case>> {
        handle_call!(
            self.contract.get_cases(skip.into(), take.into()),
            "get_cases"
        )
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

        handle_send!(
            self.contract
                .create_address(address, case_id, input.risk, input.category as u8),
            "create_address"
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

        handle_send!(
            self.contract
                .update_address(address, input.risk, input.category as u8, case_id),
            "update_address"
        )
    }

    async fn get_address(&self, address: &str) -> Result<Address> {
        let address = address.parse().map_err(|e| {
            ClientError::Ethers(format!("failed to parse address `{}`: {}", address, e))
        })?;

        handle_call!(self.contract.get_address(address), "get_address").map(|c| c.try_into())?
    }

    async fn get_address_count(&self) -> Result<u64> {
        handle_call!(self.contract.get_address_count(), "get_address_count").map(|c| c.as_u64())
    }

    async fn get_addresses(&self, skip: u64, take: u64) -> Result<Vec<Address>> {
        handle_call!(
            self.contract.get_addresses(skip.into(), take.into()),
            "get_addresses"
        )
        .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx> {
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        handle_send!(
            self.contract.create_asset(
                address,
                input.asset_id.into(),
                input.case_id.as_u128(),
                input.risk,
                input.category as u8,
            ),
            "create_asset"
        )
    }

    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx> {
        let address = input.address.parse().map_err(|e| {
            ClientError::Ethers(format!(
                "failed to parse address `{}`: {}",
                input.address, e
            ))
        })?;

        handle_send!(
            self.contract.update_asset(
                address,
                input.asset_id.into(),
                input.risk,
                input.category as u8,
                input.case_id.as_u128(),
            ),
            "update_asset"
        )
    }

    async fn get_asset(&self, address: &str, id: &AssetId) -> Result<Asset> {
        let address = address.parse().map_err(|e| {
            ClientError::Ethers(format!("failed to parse address `{}`: {}", address, e))
        })?;

        handle_call!(
            self.contract.get_asset(address, id.clone().into()),
            "get_asset"
        )
        .map(|c| c.try_into())?
    }

    async fn get_asset_count(&self) -> Result<u64> {
        handle_call!(self.contract.get_asset_count(), "get_asset_count").map(|c| c.as_u64())
    }

    async fn get_assets(&self, skip: u64, take: u64) -> Result<Vec<Asset>> {
        handle_call!(
            self.contract.get_assets(skip.into(), take.into()),
            "get_assets"
        )
        .map(|c| c.into_iter().map(|r| r.try_into()).collect())?
    }
}

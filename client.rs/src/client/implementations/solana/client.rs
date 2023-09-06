use async_trait::async_trait;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use anchor_client::{
    solana_client::rpc_filter::RpcFilterType,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    },
    Client, Cluster, Program,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

use hapi_core_solana::{accounts, instruction};

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
    },
    HapiCore,
};

use super::utils::{
    byte_array_from_str, get_address_address, get_asset_address, get_case_address,
    get_network_address, get_program_data_address, get_reporter_address, get_signer,
};

pub struct HapiCoreSolana {
    contract: Program<Arc<Keypair>>,
    network: Pubkey,
    signer: Arc<Keypair>,
}

impl HapiCoreSolana {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let program_id = options.contract_address.parse::<Pubkey>().map_err(|e| {
            ClientError::SolanaAddressParseError(format!("`contract-address`: {e}"))
        })?;

        let cluster = Cluster::from_str(&options.provider_url)
            .map_err(|e| ClientError::UrlParseError(format!("`provider-url`: {e}")))?;

        let signer = Arc::new(get_signer(options.private_key)?);

        let client = Client::new(cluster, signer.clone());
        let contract = client.program(program_id)?;

        let (network, _) = get_network_address(&options.network.to_string(), &program_id)?;

        Ok(Self {
            contract,
            network,
            signer,
        })
    }

    async fn get_reporter(&self) -> Result<(Pubkey, hapi_core_solana::Reporter)> {
        let data = self
            .contract
            .accounts::<hapi_core_solana::Reporter>(vec![RpcFilterType::DataSize(
                (hapi_core_solana::Reporter::LEN + hapi_core_solana::ACCOUNT_RESERVE_SPACE) as u64,
            )])
            .await?;

        let reporter = data
            .iter()
            .find(|(_, reporter)| reporter.account == self.signer.pubkey())
            .ok_or(ClientError::InvalidReporter)?;

        Ok(reporter.to_owned())
    }

    async fn create_network_ata(&self, token: &Pubkey) -> Result<()> {
        let cli = self.contract.async_rpc();
        let recent_blockhash = cli.get_latest_blockhash().await.unwrap();

        let create_ata_instruction = create_associated_token_account(
            &self.signer.pubkey(),
            &self.network,
            token,
            &spl_token::id(),
        );

        let create_ata_tx = Transaction::new_signed_with_payer(
            &[create_ata_instruction],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            recent_blockhash,
        );

        cli.send_and_confirm_transaction_with_spinner(&create_ata_tx)
            .await?;

        Ok(())
    }
}

macro_rules! get_account {
    ($self:expr, $address:expr, $account:ident) => {
        <$account>::try_from(
            $self
                .contract
                .account::<hapi_core_solana::$account>($address)
                .await?,
        )
    };
}

macro_rules! get_accounts {
    ($self:expr, $account:ident) => {{
        let data = $self
            .contract
            .accounts::<hapi_core_solana::$account>(vec![])
            .await?;
        let mut result = vec![];

        for (_, acc) in data {
            if acc.network == $self.network {
                result.push(<$account>::try_from(acc)?);
            }
        }

        Ok(result)
    }};
}

macro_rules! get_account_count {
    ($self:expr, $account:ident) => {
        Ok($self
            .contract
            .accounts::<hapi_core_solana::$account>(vec![])
            .await?
            .iter()
            .filter(|(_, acc)| acc.network == $self.network)
            .count() as u64)
    };
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
        let new_authority = Pubkey::from_str(address)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`new-authority`: {e}")))?;
        let program_account = self.contract.id();
        let program_data = get_program_data_address(&program_account)?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::SetAuthority {
                authority: self.signer.pubkey(),
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
        let data = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        Ok(data.authority.to_string())
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let network_data = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        let stake_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateStakeConfiguration {
                authority: self.signer.pubkey(),
                network: self.network,
                stake_mint,
            })
            .args(instruction::UpdateStakeConfiguration {
                stake_configuration: configuration.into(),
            })
            .send()
            .await?
            .to_string();

        if !network_data.stake_mint.eq(&stake_mint) {
            self.create_network_ata(&stake_mint).await?;
        }

        Ok(Tx { hash })
    }

    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        let data = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        StakeConfiguration::try_from(data)
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let network_data = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;
        let reward_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateRewardConfiguration {
                authority: self.signer.pubkey(),
                network: self.network,
                reward_mint,
            })
            .args(instruction::UpdateRewardConfiguration {
                reward_configuration: configuration.into(),
            })
            .send()
            .await?
            .to_string();

        if !network_data.reward_mint.eq(&reward_mint) {
            self.create_network_ata(&reward_mint).await?;
        }

        Ok(Tx { hash })
    }

    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        let data = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        RewardConfiguration::try_from(data)
    }

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx> {
        let (reporter, bump) = get_reporter_address(input.id, &self.network, &self.contract.id())?;
        let account = Pubkey::from_str(&input.account)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`account`: {e}")))?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::CreateReporter {
                authority: self.signer.pubkey(),
                network: self.network,
                reporter,
                system_program: system_program::id(),
            })
            .args(instruction::CreateReporter {
                reporter_id: input.id.as_u128(),
                account,
                name: input.name,
                role: input.role.into(),
                url: input.url,
                bump,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx> {
        let (reporter, _) = get_reporter_address(input.id, &self.network, &self.contract.id())?;
        let account = Pubkey::from_str(&input.account)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`account`: {e}")))?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateReporter {
                authority: self.signer.pubkey(),
                network: self.network,
                reporter,
            })
            .args(instruction::UpdateReporter {
                account,
                name: input.name,
                role: input.role.into(),
                url: input.url,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let (addr, _) =
            get_reporter_address(Uuid::from_str(id)?, &self.network, &self.contract.id())?;

        get_account!(self, addr, Reporter)
    }

    async fn get_reporter_count(&self) -> Result<u64> {
        get_account_count!(self, Reporter)
    }

    async fn get_reporters(&self, _skip: u64, _take: u64) -> Result<Vec<Reporter>> {
        get_accounts!(self, Reporter)
    }

    async fn activate_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, reporter) = self.get_reporter().await?;
        let network = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        let network_stake_token_account =
            get_associated_token_address(&self.network, &network.stake_mint);
        let reporter_stake_token_account =
            get_associated_token_address(&reporter.account, &network.stake_mint);

        let hash = self
            .contract
            .request()
            .accounts(accounts::ActivateReporter {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
                network_stake_token_account,
                reporter_stake_token_account,
                token_program: spl_token::id(),
            })
            .args(instruction::ActivateReporter)
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn deactivate_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, _) = self.get_reporter().await?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::DeactivateReporter {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
            })
            .args(instruction::DeactivateReporter)
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn unstake_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, reporter) = self.get_reporter().await?;
        let network = self
            .contract
            .account::<hapi_core_solana::Network>(self.network)
            .await?;

        let network_stake_token_account =
            get_associated_token_address(&self.network, &network.stake_mint);
        let reporter_stake_token_account =
            get_associated_token_address(&reporter.account, &network.stake_mint);

        let hash = self
            .contract
            .request()
            .accounts(accounts::Unstake {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
                network_stake_token_account,
                reporter_stake_token_account,
                token_program: spl_token::id(),
            })
            .args(instruction::Unstake)
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx> {
        let (reporter, _) = self.get_reporter().await?;
        let (case, bump) = get_case_address(input.id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::CreateCase {
                sender: self.signer.pubkey(),
                case,
                network: self.network,
                reporter,
                system_program: system_program::id(),
            })
            .args(instruction::CreateCase {
                case_id: input.id.as_u128(),
                name: input.name,
                url: input.url,
                bump,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx> {
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateCase {
                sender: self.signer.pubkey(),
                case,
                network: self.network,
                reporter,
                system_program: system_program::id(),
            })
            .args(instruction::UpdateCase {
                name: input.name,
                url: input.url,
                status: input.status.into(),
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_case(&self, id: &str) -> Result<Case> {
        let (addr, _) = get_case_address(Uuid::from_str(id)?, &self.network, &self.contract.id())?;

        get_account!(self, addr, Case)
    }

    async fn get_case_count(&self) -> Result<u64> {
        get_account_count!(self, Case)
    }

    async fn get_cases(&self, _skip: u64, _take: u64) -> Result<Vec<Case>> {
        get_accounts!(self, Case)
    }

    async fn create_address(&self, input: CreateAddressInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let (address, bump) = get_address_address(&addr, &self.network, &self.contract.id())?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::CreateAddress {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                address,
                system_program: system_program::id(),
            })
            .args(instruction::CreateAddress {
                addr,
                category: input.category.into(),
                risk_score: input.risk,
                bump,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn update_address(&self, input: UpdateAddressInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let (address, _) = get_address_address(&addr, &self.network, &self.contract.id())?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateAddress {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                address,
                system_program: system_program::id(),
            })
            .args(instruction::UpdateAddress {
                category: input.category.into(),
                risk_score: input.risk,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn get_address(&self, addr: &str) -> Result<Address> {
        let mut address = [0u8; 64];
        byte_array_from_str(addr, &mut address)?;

        let (addr, _) = get_address_address(&address, &self.network, &self.contract.id())?;

        get_account!(self, addr, Address)
    }

    async fn get_address_count(&self) -> Result<u64> {
        get_account_count!(self, Address)
    }

    async fn get_addresses(&self, _skip: u64, _take: u64) -> Result<Vec<Address>> {
        get_accounts!(self, Address)
    }

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let mut asset_id = [0u8; 64];
        byte_array_from_str(&input.asset_id.to_string(), &mut asset_id)?;

        let (asset, bump) =
            get_asset_address(&addr, &asset_id, &self.network, &self.contract.id())?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::CreateAsset {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                asset,
                system_program: system_program::id(),
            })
            .args(instruction::CreateAsset {
                addr,
                asset_id,
                category: input.category.into(),
                risk_score: input.risk,
                bump,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let mut asset_id = [0u8; 64];
        byte_array_from_str(&input.asset_id.to_string(), &mut asset_id)?;

        let (asset, _) = get_asset_address(&addr, &asset_id, &self.network, &self.contract.id())?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.contract.id())?;

        let hash = self
            .contract
            .request()
            .accounts(accounts::UpdateAsset {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                asset,
                system_program: system_program::id(),
            })
            .args(instruction::UpdateAsset {
                category: input.category.into(),
                risk_score: input.risk,
            })
            .send()
            .await?
            .to_string();

        Ok(Tx { hash })
    }
    async fn get_asset(&self, address: &str, id: &AssetId) -> Result<Asset> {
        let mut asset_address = [0u8; 64];
        byte_array_from_str(address, &mut asset_address)?;

        let mut asset_id = [0u8; 64];
        byte_array_from_str(&id.to_string(), &mut asset_id)?;

        let (addr, _) = get_asset_address(
            &asset_address,
            &asset_id,
            &self.network,
            &self.contract.id(),
        )?;

        get_account!(self, addr, Asset)
    }
    async fn get_asset_count(&self) -> Result<u64> {
        get_account_count!(self, Asset)
    }
    async fn get_assets(&self, _skip: u64, _take: u64) -> Result<Vec<Asset>> {
        get_accounts!(self, Asset)
    }
}

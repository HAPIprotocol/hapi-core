use {
    anchor_client::{
        anchor_lang::{AccountDeserialize, Discriminator, InstructionData, ToAccountMetas},
        solana_client::{
            nonblocking::rpc_client::RpcClient,
            rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
            rpc_filter::{Memcmp, RpcFilterType},
        },
        solana_sdk::{
            commitment_config::CommitmentConfig,
            pubkey::Pubkey,
            signature::{Keypair, Signer},
            system_program,
            transaction::Transaction,
        },
        RequestBuilder,
    },
    async_trait::async_trait,
    hapi_core_solana::{accounts, instruction},
    solana_account_decoder::UiAccountEncoding,
    spl_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account,
    },
    spl_token::solana_program::instruction::Instruction,
    std::{str::FromStr, sync::Arc, time::Duration},
    uuid::Uuid,
};

use crate::{
    client::{
        configuration::{RewardConfiguration, StakeConfiguration},
        entities::{
            address::{Address, ConfirmAddressInput, CreateAddressInput, UpdateAddressInput},
            asset::{Asset, AssetId, ConfirmAssetInput, CreateAssetInput, UpdateAssetInput},
            case::{Case, CreateCaseInput, UpdateCaseInput},
            reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        },
        interface::HapiCoreOptions,
        result::{ClientError, Result, Tx},
    },
    get_solana_account, get_solana_account_count, get_solana_accounts, HapiCore,
};

use super::{
    instruction_data::get_hapi_sighashes,
    utils::{
        byte_array_from_str, get_address_address, get_asset_address, get_case_address,
        get_confirmation_address, get_network_address, get_program_data_address,
        get_reporter_address, get_signer,
    },
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct HapiCoreSolana {
    pub rpc_client: RpcClient,
    pub program_id: Pubkey,
    network: Pubkey,
    signer: Arc<Keypair>,
    pub(crate) hashes: Vec<[u8; 8]>,
}

impl HapiCoreSolana {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let program_id = options.contract_address.parse::<Pubkey>().map_err(|e| {
            ClientError::SolanaAddressParseError(format!("`contract-address`: {e}"))
        })?;

        let signer = Arc::new(get_signer(options.private_key)?);

        let (network, _) = get_network_address(&options.network.to_string(), &program_id)?;

        let rpc_client = RpcClient::new_with_timeout(options.provider_url.clone(), DEFAULT_TIMEOUT);

        let hashes = get_hapi_sighashes();

        Ok(Self {
            rpc_client,
            program_id,
            network,
            signer,
            hashes,
        })
    }

    async fn send_transaction(&self, instructions: &[Instruction]) -> Result<Tx> {
        let latest_hash = self.rpc_client.get_latest_blockhash().await?;

        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&self.signer.pubkey()),
            &vec![&self.signer.clone()],
            latest_hash,
        );

        let hash = self
            .rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .unwrap()
            .to_string();

        Ok(Tx { hash })
    }

    pub async fn get_account_data<T: AccountDeserialize>(&self, address: &Pubkey) -> Result<T> {
        let mut data: &[u8] = &self
            .rpc_client
            .get_account_with_commitment(address, CommitmentConfig::processed())
            .await?
            .value
            .ok_or(ClientError::AccountNotFound)?
            .data;

        T::try_deserialize(&mut data)
            .map_err(|e| ClientError::AccountDeserializationError(e.to_string()))
    }

    async fn get_accounts<T>(&self, data_size: usize) -> Result<Vec<(Pubkey, T)>>
    where
        T: AccountDeserialize + Discriminator,
    {
        let account_type_filter =
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(0, &T::discriminator()));

        let account_size_filter =
            RpcFilterType::DataSize((data_size + hapi_core_solana::ACCOUNT_RESERVE_SPACE) as u64);

        let config = RpcProgramAccountsConfig {
            filters: Some(vec![account_type_filter, account_size_filter]),
            account_config: RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                ..RpcAccountInfoConfig::default()
            },
            ..RpcProgramAccountsConfig::default()
        };

        let accounts = self
            .rpc_client
            .get_program_accounts_with_config(&self.program_id, config)
            .await?;

        let mut result = vec![];

        for (pubkey, acc) in accounts {
            let account = T::try_deserialize(&mut (&acc.data as &[u8]))
                .map_err(|e| ClientError::AccountDeserializationError(e.to_string()))?;

            result.push((pubkey, account))
        }

        Ok(result)
    }

    async fn call_contract(
        &self,
        accounts: impl ToAccountMetas,
        args: impl InstructionData,
    ) -> Result<Tx> {
        let instructions = RequestBuilder::from(
            self.program_id,
            &self.rpc_client.url(),
            self.signer.clone(),
            None,
        )
        .accounts(accounts)
        .args(args)
        .instructions()?;

        self.send_transaction(&instructions).await
    }

    async fn get_reporter(&self) -> Result<(Pubkey, hapi_core_solana::Reporter)> {
        let data = self
            .get_accounts::<hapi_core_solana::Reporter>(hapi_core_solana::Reporter::LEN)
            .await?;

        let reporter = data
            .iter()
            .find(|(_, reporter)| reporter.account == self.signer.pubkey())
            .ok_or(ClientError::InvalidReporter)?;

        Ok(reporter.to_owned())
    }

    async fn create_network_ata(&self, token: &Pubkey) -> Result<()> {
        let create_ata_instruction = create_associated_token_account(
            &self.signer.pubkey(),
            &self.network,
            token,
            &spl_token::id(),
        );

        self.send_transaction(&[create_ata_instruction]).await?;

        Ok(())
    }
}

#[async_trait]
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
        let program_account = self.program_id;
        let program_data = get_program_data_address(&program_account)?;

        self.call_contract(
            accounts::SetAuthority {
                authority: self.signer.pubkey(),
                network: self.network,
                new_authority,
                program_account,
                program_data,
            },
            instruction::SetAuthority,
        )
        .await
    }

    async fn get_authority(&self) -> Result<String> {
        let account = self
            .get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?;

        Ok(account.authority.to_string())
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let network_data = self
            .get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?;

        let stake_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let result = self
            .call_contract(
                accounts::UpdateStakeConfiguration {
                    authority: self.signer.pubkey(),
                    network: self.network,
                    stake_mint,
                },
                instruction::UpdateStakeConfiguration {
                    stake_configuration: configuration.into(),
                },
            )
            .await;

        if !network_data.stake_mint.eq(&stake_mint) {
            self.create_network_ata(&stake_mint).await?;
        }

        result
    }

    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        self.get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?
            .try_into()
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let network_data = self
            .get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?;

        let reward_mint = Pubkey::from_str(&configuration.token)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`stake-token`: {e}")))?;

        let result = self
            .call_contract(
                accounts::UpdateRewardConfiguration {
                    authority: self.signer.pubkey(),
                    network: self.network,
                    reward_mint,
                },
                instruction::UpdateRewardConfiguration {
                    reward_configuration: configuration.into(),
                },
            )
            .await;

        if !network_data.reward_mint.eq(&reward_mint) {
            self.create_network_ata(&reward_mint).await?;
        }

        result
    }

    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        self.get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?
            .try_into()
    }

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx> {
        let (reporter, bump) = get_reporter_address(input.id, &self.network, &self.program_id)?;
        let account = Pubkey::from_str(&input.account)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`account`: {e}")))?;

        self.call_contract(
            accounts::CreateReporter {
                authority: self.signer.pubkey(),
                network: self.network,
                reporter,
                system_program: system_program::id(),
            },
            instruction::CreateReporter {
                reporter_id: input.id.as_u128(),
                account,
                name: input.name,
                role: input.role.into(),
                url: input.url,
                bump,
            },
        )
        .await
    }

    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx> {
        let (reporter, _) = get_reporter_address(input.id, &self.network, &self.program_id)?;
        let account = Pubkey::from_str(&input.account)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`account`: {e}")))?;

        self.call_contract(
            accounts::UpdateReporter {
                authority: self.signer.pubkey(),
                network: self.network,
                reporter,
            },
            instruction::UpdateReporter {
                account,
                name: input.name,
                role: input.role.into(),
                url: input.url,
            },
        )
        .await
    }

    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let (addr, _) = get_reporter_address(Uuid::from_str(id)?, &self.network, &self.program_id)?;

        get_solana_account!(self, &addr, Reporter)
    }

    async fn get_reporter_count(&self) -> Result<u64> {
        get_solana_account_count!(self, Reporter)
    }

    async fn get_reporters(&self, _skip: u64, _take: u64) -> Result<Vec<Reporter>> {
        get_solana_accounts!(self, Reporter)
    }

    async fn activate_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, reporter) = self.get_reporter().await?;
        let network_data = self
            .get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?;

        let network_stake_token_account =
            get_associated_token_address(&self.network, &network_data.stake_mint);
        let reporter_stake_token_account =
            get_associated_token_address(&reporter.account, &network_data.stake_mint);

        self.call_contract(
            accounts::ActivateReporter {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
                network_stake_token_account,
                reporter_stake_token_account,
                token_program: spl_token::id(),
            },
            instruction::ActivateReporter,
        )
        .await
    }

    async fn deactivate_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, _) = self.get_reporter().await?;

        self.call_contract(
            accounts::DeactivateReporter {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
            },
            instruction::DeactivateReporter,
        )
        .await
    }

    async fn unstake_reporter(&self) -> Result<Tx> {
        let (reporter_pubkey, reporter) = self.get_reporter().await?;
        let network_data = self
            .get_account_data::<hapi_core_solana::Network>(&self.network)
            .await?;

        let network_stake_token_account =
            get_associated_token_address(&self.network, &network_data.stake_mint);
        let reporter_stake_token_account =
            get_associated_token_address(&reporter.account, &network_data.stake_mint);

        self.call_contract(
            accounts::Unstake {
                signer: self.signer.pubkey(),
                network: self.network,
                reporter: reporter_pubkey,
                network_stake_token_account,
                reporter_stake_token_account,
                token_program: spl_token::id(),
            },
            instruction::Unstake,
        )
        .await
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx> {
        let (reporter, _) = self.get_reporter().await?;
        let (case, bump) = get_case_address(input.id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::CreateCase {
                sender: self.signer.pubkey(),
                case,
                network: self.network,
                reporter,
                system_program: system_program::id(),
            },
            instruction::CreateCase {
                case_id: input.id.as_u128(),
                name: input.name,
                url: input.url,
                bump,
            },
        )
        .await
    }

    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx> {
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::UpdateCase {
                sender: self.signer.pubkey(),
                case,
                network: self.network,
                reporter,
                system_program: system_program::id(),
            },
            instruction::UpdateCase {
                name: input.name,
                url: input.url,
                status: input.status.into(),
            },
        )
        .await
    }

    async fn get_case(&self, id: &str) -> Result<Case> {
        let (addr, _) = get_case_address(Uuid::from_str(id)?, &self.network, &self.program_id)?;

        get_solana_account!(self, &addr, Case)
    }

    async fn get_case_count(&self) -> Result<u64> {
        get_solana_account_count!(self, Case)
    }

    async fn get_cases(&self, _skip: u64, _take: u64) -> Result<Vec<Case>> {
        get_solana_accounts!(self, Case)
    }

    async fn create_address(&self, input: CreateAddressInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let (address, bump) = get_address_address(&addr, &self.network, &self.program_id)?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::CreateAddress {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                address,
                system_program: system_program::id(),
            },
            instruction::CreateAddress {
                addr,
                category: input.category.into(),
                risk_score: input.risk,
                bump,
            },
        )
        .await
    }

    async fn update_address(&self, input: UpdateAddressInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let (address, _) = get_address_address(&addr, &self.network, &self.program_id)?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::UpdateAddress {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                address,
                system_program: system_program::id(),
            },
            instruction::UpdateAddress {
                category: input.category.into(),
                risk_score: input.risk,
            },
        )
        .await
    }

    async fn confirm_address(&self, input: ConfirmAddressInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let (address, _) = get_address_address(&addr, &self.network, &self.program_id)?;
        let address_data = get_solana_account!(self, &address, Address)?;

        let (reporter, _) = self.get_reporter().await?;
        let reporter_data = get_solana_account!(self, &reporter, Reporter)?;

        let (case, _) = get_case_address(address_data.case_id, &self.network, &self.program_id)?;
        let (confirmation, bump) =
            get_confirmation_address(&address, reporter_data.id, &self.program_id)?;

        self.call_contract(
            accounts::ConfirmAddress {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                address,
                confirmation,
                system_program: system_program::id(),
            },
            instruction::ConfirmAddress { bump },
        )
        .await
    }

    async fn get_address(&self, addr: &str) -> Result<Address> {
        let mut address = [0u8; 64];
        byte_array_from_str(addr, &mut address)?;

        let (addr, _) = get_address_address(&address, &self.network, &self.program_id)?;

        get_solana_account!(self, &addr, Address)
    }

    async fn get_address_count(&self) -> Result<u64> {
        get_solana_account_count!(self, Address)
    }

    async fn get_addresses(&self, _skip: u64, _take: u64) -> Result<Vec<Address>> {
        get_solana_accounts!(self, Address)
    }

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let mut asset_id = [0u8; 32];
        byte_array_from_str(&input.asset_id.to_string(), &mut asset_id)?;

        let (asset, bump) = get_asset_address(&addr, &asset_id, &self.network, &self.program_id)?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::CreateAsset {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                asset,
                system_program: system_program::id(),
            },
            instruction::CreateAsset {
                addr,
                asset_id,
                category: input.category.into(),
                risk_score: input.risk,
                bump,
            },
        )
        .await
    }

    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let mut asset_id = [0u8; 32];
        byte_array_from_str(&input.asset_id.to_string(), &mut asset_id)?;

        let (asset, _) = get_asset_address(&addr, &asset_id, &self.network, &self.program_id)?;
        let (reporter, _) = self.get_reporter().await?;
        let (case, _) = get_case_address(input.case_id, &self.network, &self.program_id)?;

        self.call_contract(
            accounts::UpdateAsset {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                asset,
                system_program: system_program::id(),
            },
            instruction::UpdateAsset {
                category: input.category.into(),
                risk_score: input.risk,
            },
        )
        .await
    }

    async fn confirm_asset(&self, input: ConfirmAssetInput) -> Result<Tx> {
        let mut addr = [0u8; 64];
        byte_array_from_str(&input.address, &mut addr)?;

        let mut asset_id = [0u8; 32];
        byte_array_from_str(&input.asset_id.to_string(), &mut asset_id)?;

        let (asset, _) = get_asset_address(&addr, &asset_id, &self.network, &self.program_id)?;
        let asset_data = get_solana_account!(self, &asset, Asset)?;

        let (reporter, _) = self.get_reporter().await?;
        let reporter_data = get_solana_account!(self, &reporter, Reporter)?;

        let (case, _) = get_case_address(asset_data.case_id, &self.network, &self.program_id)?;
        let (confirmation, bump) =
            get_confirmation_address(&asset, reporter_data.id, &self.program_id)?;

        self.call_contract(
            accounts::ConfirmAsset {
                sender: self.signer.pubkey(),
                network: self.network,
                reporter,
                case,
                asset,
                confirmation,
                system_program: system_program::id(),
            },
            instruction::ConfirmAsset { bump },
        )
        .await
    }

    async fn get_asset(&self, address: &str, id: &AssetId) -> Result<Asset> {
        let mut asset_address = [0u8; 64];
        byte_array_from_str(address, &mut asset_address)?;

        let mut asset_id = [0u8; 32];
        byte_array_from_str(&id.to_string(), &mut asset_id)?;

        let (addr, _) =
            get_asset_address(&asset_address, &asset_id, &self.network, &self.program_id)?;

        get_solana_account!(self, &addr, Asset)
    }
    async fn get_asset_count(&self) -> Result<u64> {
        get_solana_account_count!(self, Asset)
    }
    async fn get_assets(&self, _skip: u64, _take: u64) -> Result<Vec<Asset>> {
        get_solana_accounts!(self, Asset)
    }
}

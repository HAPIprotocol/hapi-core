use async_trait::async_trait;
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_client::{
    methods::{self, broadcast_tx_async::RpcBroadcastTxAsyncRequest, query::RpcQueryRequest},
    JsonRpcClient,
};
use near_jsonrpc_primitives::types::{
    query::{QueryResponseKind, RpcQueryResponse},
    transactions::TransactionInfo,
};
use near_primitives::{
    transaction::{Action, FunctionCallAction, Transaction},
    types::{AccountId, BlockReference, Finality, FunctionArgs},
    views::{FinalExecutionStatus, QueryRequest},
};
use serde::Deserialize;
use serde_json::{from_slice, json, Value};
use tokio::{time, time::Duration};
use uuid::Uuid;

use hapi_core_near::{
    AddressView as NearAddress, AssetView as NearAsset, Case as NearCase, Reporter as NearReporter,
};

pub const TRANSACTION_TIMEOUT: Duration = Duration::from_secs(60);
pub const PERIOD_CHECK_TX_STATUS: Duration = Duration::from_secs(2);
pub const DELAY_AFTER_TX_EXECUTION: Duration = Duration::from_secs(1);

use crate::{
    client::{
        configuration::{RewardConfiguration, StakeConfiguration},
        entities::{
            address::{Address, CreateAddressInput, UpdateAddressInput},
            asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
            case::{Case, CreateCaseInput, UpdateCaseInput},
            reporter::{CreateReporterInput, Reporter, ReporterRole, UpdateReporterInput},
        },
        near::GAS_FOR_TX,
        result::{ClientError, Result, Tx},
    },
    HapiCore, HapiCoreOptions,
};

pub struct HapiCoreNear {
    client: JsonRpcClient,
    contract_address: AccountId,
    signer: Option<String>,
    account_id: Option<String>,
}

impl HapiCoreNear {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let client = JsonRpcClient::connect(options.provider_url.as_str());
        let signer = options.private_key;
        let account_id = options.account_id;

        Ok(Self {
            client,
            contract_address: options.contract_address.try_into()?,
            signer,
            account_id,
        })
    }
}

#[macro_export]
macro_rules! uuid_to_u128 {
    ($id:expr) => {
        Uuid::parse_str(&$id.to_string())?.as_u128().to_string()
    };
}

macro_rules! build_tx {
    ($self:expr, $signer:expr, $access_key:expr, $method:expr, $args:expr) => {
        Transaction {
            signer_id: $signer.account_id.clone(),
            public_key: $signer.public_key.clone(),
            nonce: $self.get_nonce(&$access_key)? + 1,
            receiver_id: $self.contract_address.clone(),
            block_hash: $access_key.block_hash,
            actions: vec![Action::FunctionCall(FunctionCallAction {
                method_name: $method.to_string(),
                args: $args.to_string().into_bytes(),
                gas: GAS_FOR_TX,
                deposit: 0,
            })],
        }
    };
}

pub(crate) async fn execute_transaction(
    transaction: Transaction,
    signer: InMemorySigner,
    client: &JsonRpcClient,
) -> Result<Tx> {
    let request = RpcBroadcastTxAsyncRequest {
        signed_transaction: transaction.sign(&signer),
    };
    let sent_at = time::Instant::now();
    let tx_hash = client.call(request).await?;

    loop {
        if time::Instant::now() > sent_at + TRANSACTION_TIMEOUT {
            return Err(ClientError::TimeoutError("Transaction timeout".to_string()));
        }

        let response = client
            .call(methods::tx::RpcTransactionStatusRequest {
                transaction_info: TransactionInfo::TransactionId {
                    hash: tx_hash.clone(),
                    account_id: signer.account_id.clone(),
                },
            })
            .await;

        match response {
            Err(err) => match err.handler_error() {
                Some(methods::tx::RpcTransactionError::UnknownTransaction { .. }) => {
                    time::sleep(PERIOD_CHECK_TX_STATUS).await;
                    continue;
                }
                _ => Err(err)?,
            },
            Ok(response) => match response.status {
                FinalExecutionStatus::SuccessValue(_) => {
                    time::sleep(DELAY_AFTER_TX_EXECUTION).await;
                    break;
                }
                FinalExecutionStatus::Failure(err) => Err(ClientError::InvalidResponse(format!(
                    "Call method failed with {err}"
                )))?,
                _ => {
                    continue;
                }
            },
        }
    }

    Ok(Tx {
        hash: tx_hash.to_string(),
    })
}

#[async_trait(?Send)]
impl HapiCore for HapiCoreNear {
    fn is_valid_address(&self, address: &str) -> Result<()> {
        AccountId::try_from(address.to_string())?;
        Ok(())
    }

    async fn set_authority(&self, address: &str) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "set_authority",
            json!({
                "authority": address,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_authority(&self) -> Result<String> {
        let request = self.view_request("get_authority", None);

        Ok(self.get_response::<String>(request).await?)
    }

    async fn update_stake_configuration(&self, configuration: StakeConfiguration) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_stake_configuration",
            json!({
                "stake_configuration": configuration,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_stake_configuration(&self) -> Result<StakeConfiguration> {
        let request = self.view_request("get_stake_configuration", None);

        Ok(self.get_response::<StakeConfiguration>(request).await?)
    }

    async fn update_reward_configuration(&self, configuration: RewardConfiguration) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_reward_configuration",
            json!({
                "reward_configuration": configuration,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_reward_configuration(&self) -> Result<RewardConfiguration> {
        let request = self.view_request("get_reward_configuration", None);

        Ok(self.get_response::<RewardConfiguration>(request).await?)
    }

    async fn create_reporter(&self, input: CreateReporterInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "create_reporter",
            json!({
                "id": uuid_to_u128!(input.id),
                "account_id": input.account,
                "name": input.name,
                "role": input.role,
                "url": input.url,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn update_reporter(&self, input: UpdateReporterInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_reporter",
            json!({
                "id": uuid_to_u128!(input.id),
                "account_id": input.account,
                "name": input.name,
                "role": input.role,
                "url": input.url,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let request = self.view_request("get_reporter", Some(json!({ "id": uuid_to_u128!(id) })));

        let reporter = self.get_response::<NearReporter>(request).await?;

        reporter.try_into()
    }

    async fn get_reporter_count(&self) -> Result<u64> {
        let request = self.view_request("get_reporter_count", None);

        Ok(self.get_response::<u64>(request).await?)
    }

    async fn get_reporters(&self, skip: u64, take: u64) -> Result<Vec<Reporter>> {
        let request =
            self.view_request("get_reporters", Some(json!({ "skip": skip, "take": take })));

        let reporter = self.get_response::<Vec<NearReporter>>(request).await?;

        Ok(reporter
            .into_iter()
            .map(|reporter| reporter.try_into())
            .collect::<Result<Vec<Reporter>>>()?)
    }

    /// This method calls ft_transfer_call method of the token contract.
    async fn activate_reporter(&self) -> Result<Tx> {
        let signer = self.get_signer()?;

        // get reporter role
        let reporter_role = {
            let request = self.view_request(
                "get_reporter_by_account",
                Some(json!({ "account_id": signer.account_id.clone() })),
            );

            TryInto::<Reporter>::try_into(self.get_response::<NearReporter>(request).await?)?.role
        };

        // get stake configuration
        let (stake_token, stake_amount) = {
            let request = self.view_request("get_stake_configuration", None);

            let stake_config = self.get_response::<StakeConfiguration>(request).await?;
            let stake_amount = match reporter_role {
                ReporterRole::Validator => stake_config.validator_stake,
                ReporterRole::Tracer => stake_config.tracer_stake,
                ReporterRole::Publisher => stake_config.publisher_stake,
                ReporterRole::Authority => stake_config.authority_stake,
            };
            let stake_token: AccountId = stake_config.token.try_into()?;

            (stake_token, stake_amount)
        };

        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        // ft_transfer_call to activate reporter
        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce: self.get_nonce(&access_key_query_response)? + 1,
            receiver_id: stake_token.clone(),
            block_hash: access_key_query_response.block_hash,
            actions: vec![Action::FunctionCall(FunctionCallAction {
                method_name: "ft_transfer_call".to_string(),
                args: json!({"receiver_id": self.contract_address, "amount": stake_amount, "msg": "", "memo": ""}).to_string().into_bytes(),
                gas: 50_000_000_000_000, // 50 TeraGas
                deposit: 1,
            })],
        };

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn deactivate_reporter(&self) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "deactivate_reporter",
            ""
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn unstake_reporter(&self) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(self, signer, access_key_query_response, "unstake", "");

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn create_case(&self, input: CreateCaseInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "create_case",
            json!({
                "id": uuid_to_u128!(input.id),
                "name": input.name,
                "url": input.url,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn update_case(&self, input: UpdateCaseInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_case",
            json!({
                "id": uuid_to_u128!(input.id),
                "name": input.name,
                "status": input.status,
                "url": input.url,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_case(&self, id: &str) -> Result<Case> {
        let request = self.view_request("get_case", Some(json!({ "id": uuid_to_u128!(id) })));

        Ok(self.get_response::<NearCase>(request).await?.try_into()?)
    }

    async fn get_case_count(&self) -> Result<u64> {
        let request = self.view_request("get_case_count", None);

        Ok(self.get_response::<u64>(request).await?)
    }

    async fn get_cases(&self, skip: u64, take: u64) -> Result<Vec<Case>> {
        let request = self.view_request("get_cases", Some(json!({ "skip": skip, "take": take })));

        Ok(self
            .get_response::<Vec<NearCase>>(request)
            .await?
            .into_iter()
            .map(|case| case.try_into())
            .collect::<Result<Vec<Case>>>()?)
    }

    async fn create_address(&self, input: CreateAddressInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "create_address",
            json!({
                "address": input.address,
                "category": input.category,
                "case_id": uuid_to_u128!(input.case_id),
                "risk_score": input.risk,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn update_address(&self, input: UpdateAddressInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;
        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_address",
            json!({
                "address": input.address,
                "category": input.category,
                "case_id": uuid_to_u128!(input.case_id),
                "risk_score": input.risk,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_address(&self, addr: &str) -> Result<Address> {
        let request = self.view_request("get_address", Some(json!({ "address": addr })));

        Ok(self
            .get_response::<NearAddress>(request)
            .await?
            .try_into()?)
    }

    async fn get_address_count(&self) -> Result<u64> {
        let request = self.view_request("get_address_count", None);

        Ok(self.get_response::<u64>(request).await?)
    }

    async fn get_addresses(&self, skip: u64, take: u64) -> Result<Vec<Address>> {
        let request =
            self.view_request("get_addresses", Some(json!({ "skip": skip, "take": take })));

        Ok(self
            .get_response::<Vec<NearAddress>>(request)
            .await?
            .into_iter()
            .map(|address| address.try_into())
            .collect::<Result<Vec<Address>>>()?)
    }

    async fn create_asset(&self, input: CreateAssetInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;

        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "create_asset",
            json!({
                "address": input.address,
                "id": input.asset_id,
                "category": input.category,
                "case_id": uuid_to_u128!(input.case_id),
                "risk_score": input.risk,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn update_asset(&self, input: UpdateAssetInput) -> Result<Tx> {
        let signer = self.get_signer()?;
        let access_key_query_response: RpcQueryResponse = self.get_access_key(&signer).await?;
        let transaction = build_tx!(
            self,
            signer,
            access_key_query_response,
            "update_asset",
            json!({
                "address": input.address,
                "id": input.asset_id,
                "category": input.category,
                "case_id": uuid_to_u128!(input.case_id),
                "risk_score": input.risk,
            })
        );

        Ok(execute_transaction(transaction, signer, &self.client).await?)
    }

    async fn get_asset(&self, address: &str, id: &AssetId) -> Result<Asset> {
        let request = self.view_request("get_asset", Some(json!({ "address": address, "id": id })));

        Ok(self.get_response::<NearAsset>(request).await?.try_into()?)
    }

    async fn get_asset_count(&self) -> Result<u64> {
        let request = self.view_request("get_asset_count", None);

        Ok(self.get_response::<u64>(request).await?)
    }

    async fn get_assets(&self, skip: u64, take: u64) -> Result<Vec<Asset>> {
        let request = self.view_request("get_assets", Some(json!({ "skip": skip, "take": take })));

        Ok(self
            .get_response::<Vec<NearAsset>>(request)
            .await?
            .into_iter()
            .map(|asset| asset.try_into())
            .collect::<Result<Vec<Asset>>>()?)
    }
}

impl HapiCoreNear {
    pub fn view_request(&self, method: &str, args: Option<Value>) -> RpcQueryRequest {
        RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: self.contract_address.clone(),
                method_name: method.to_string(),
                args: FunctionArgs::from(args.unwrap_or_default().to_string().into_bytes()),
            },
        }
    }
    pub async fn get_response<T: for<'a> Deserialize<'a>>(
        &self,
        request: RpcQueryRequest,
    ) -> Result<T> {
        let result = self.client.call(request).await?;
        if let QueryResponseKind::CallResult(result) = result.kind {
            Ok(from_slice::<T>(&result.result)?)
        } else {
            Err(ClientError::InvalidResponse(
                "failed to receive call result".into(),
            ))
        }
    }

    fn get_signer(&self) -> Result<InMemorySigner> {
        let signer_secret_key: SecretKey = self
            .signer
            .as_ref()
            .ok_or(ClientError::SignerError)?
            .parse()
            .map_err(|_| ClientError::SignerError)?;
        let signer_account_id = self
            .account_id
            .as_ref()
            .ok_or(ClientError::SignerError)?
            .clone()
            .try_into()?;
        Ok(near_crypto::InMemorySigner::from_secret_key(
            signer_account_id,
            signer_secret_key,
        ))
    }

    async fn get_access_key(&self, signer: &InMemorySigner) -> Result<RpcQueryResponse> {
        Ok(self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: signer.public_key.clone(),
                },
            })
            .await?)
    }
    fn get_nonce(&self, access_key_request: &RpcQueryResponse) -> Result<u64> {
        match &access_key_request.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(access_key.nonce),
            _ => Err(ClientError::InvalidResponse(
                "failed to extract current nonce".into(),
            )),
        }
    }
}

use crate::{client::result::ClientError, HapiCoreOptions};
use async_trait::async_trait;
use hapi_core_near::Reporter as NearReporter;
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_primitives::types::{
    query::{QueryResponseKind, RpcQueryResponse},
    transactions::TransactionInfo,
};
use serde::Deserialize;
use tokio::time;
use uuid::Uuid;

mod conversion;

use crate::{
    client::{
        address::{Address, CreateAddressInput, UpdateAddressInput},
        asset::{Asset, AssetId, CreateAssetInput, UpdateAssetInput},
        case::{Case, CreateCaseInput, UpdateCaseInput},
        configuration::{RewardConfiguration, StakeConfiguration},
        reporter::{CreateReporterInput, Reporter, UpdateReporterInput},
        result::{Result, Tx},
        token::TokenContract,
    },
    Amount, HapiCore,
};

use near_jsonrpc_client::{
    methods::{self, query::RpcQueryRequest},
    JsonRpcClient,
};
use near_primitives::{
    transaction::Transaction,
    types::{AccountId, BlockReference, Finality, FunctionArgs},
    views::FinalExecutionStatus,
};
use near_primitives::{
    transaction::{Action, FunctionCallAction},
    views::QueryRequest,
};

use serde_json::{from_slice, json, Value};

pub struct Account {
    pub account_id: AccountId,
    pub secret_key: SecretKey,
}

pub struct HapiCoreNear {
    client: JsonRpcClient,
    contract_address: AccountId,
    signer: Option<SecretKey>,
    account_id: Option<String>,
}

impl HapiCoreNear {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let rpc_address = "http://localhost:3030";
        let client = JsonRpcClient::connect(rpc_address);
        let signer = options.private_key.map(|key| key.parse().unwrap());
        let account_id = options.account_id;

        Ok(Self {
            client,
            contract_address: AccountId::try_from(options.contract_address)?,
            signer,
            account_id,
        })
    }
}

macro_rules! transform_id {
    ($id:expr) => {
        Uuid::parse_str(&$id.to_string())
            .unwrap()
            .as_u128()
            .to_string()
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
                gas: 10_000_000_000_000, // 10 TeraGas
                deposit: 0,
            })],
        }
    };
}

macro_rules! wait_tx_execution {
    ($tx_hash:expr, $signer:expr, $sent_at:expr, $client:expr ) => {
        loop {
            let response = $client
                .call(methods::tx::RpcTransactionStatusRequest {
                    transaction_info: TransactionInfo::TransactionId {
                        hash: $tx_hash,
                        account_id: $signer.account_id.clone(),
                    },
                })
                .await;
            let received_at = time::Instant::now();
            let delta = (received_at - $sent_at).as_secs();

            if delta > 60 {
                return Err(ClientError::TimeoutError("Transaction timeout".to_string()));
            }

            match response {
                Err(err) => match err.handler_error() {
                    Some(methods::tx::RpcTransactionError::UnknownTransaction { .. }) => {
                        time::sleep(time::Duration::from_secs(2)).await;
                        continue;
                    }
                    _ => Err(err)?,
                },
                Ok(response) => match response.status {
                    FinalExecutionStatus::SuccessValue(_) => {
                        break;
                    }
                    FinalExecutionStatus::Failure(err) => {
                        return Err(ClientError::InvalidResponse(format!(
                            "Call method failed with {err}"
                        )));
                    }
                    _ => {
                        continue;
                    }
                },
            }
        }
    };
}

#[async_trait]
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

        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: transaction.sign(&signer),
        };
        let sent_at = time::Instant::now();
        let tx_hash = self.client.call(request).await?;

        wait_tx_execution!(tx_hash, signer, sent_at, self.client);

        Ok(Tx {
            hash: format!("{:?}", tx_hash),
        })
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

        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: transaction.sign(&signer),
        };
        let sent_at = time::Instant::now();
        let tx_hash = self.client.call(request).await?;

        wait_tx_execution!(tx_hash, signer, sent_at, self.client);

        Ok(Tx {
            hash: format!("{:?}", tx_hash),
        })
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

        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: transaction.sign(&signer),
        };
        let sent_at = time::Instant::now();
        let tx_hash = self.client.call(request).await?;

        wait_tx_execution!(tx_hash, signer, sent_at, self.client);

        Ok(Tx {
            hash: format!("{:?}", tx_hash),
        })
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
                "id": transform_id!(input.id),
                "account_id": input.account,
                "name": input.name,
                "role": input.role,
                "url": input.url,
            })
        );

        let request = methods::broadcast_tx_async::RpcBroadcastTxAsyncRequest {
            signed_transaction: transaction.sign(&signer),
        };
        let sent_at = time::Instant::now();
        let tx_hash = self.client.call(request).await?;

        wait_tx_execution!(tx_hash, signer, sent_at, self.client);

        Ok(Tx {
            hash: format!("{:?}", tx_hash),
        })
    }
    async fn update_reporter(&self, _input: UpdateReporterInput) -> Result<Tx> {
        unimplemented!()
    }
    async fn get_reporter(&self, id: &str) -> Result<Reporter> {
        let request = self.view_request("get_reporter", Some(json!({ "id": transform_id!(id) })));

        let reporter = self.get_response::<NearReporter>(request).await?;

        reporter.try_into()
    }
    async fn get_reporter_count(&self) -> Result<u64> {
        let request = self.view_request("get_reporter_count", None);

        Ok(self.get_response::<u64>(request).await?)
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
        let request = self.view_request("get_case_count", None);

        Ok(self.get_response::<u64>(request).await?)
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

impl HapiCoreNear {
    pub fn view_request(&self, method: &str, args: Option<Value>) -> RpcQueryRequest {
        RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: self.contract_address.parse().unwrap(),
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
            return Ok(from_slice::<T>(&result.result)?);
        } else {
            Err(ClientError::InvalidResponse(
                "failed to receive call result".into(),
            ))
        }
    }

    fn get_signer(&self) -> Result<InMemorySigner> {
        let signer_secret_key = self.signer.as_ref().ok_or(ClientError::SignerError)?;
        let signer_account_id = AccountId::try_from(self.account_id.as_ref().unwrap().clone())?;
        Ok(near_crypto::InMemorySigner::from_secret_key(
            signer_account_id,
            signer_secret_key.clone(),
        ))
    }
    async fn get_access_key(&self, signer: &InMemorySigner) -> Result<RpcQueryResponse> {
        Ok(self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: signer.public_key.clone(),
                },
            })
            .await?)
    }
    fn get_nonce(&self, access_key_request: &RpcQueryResponse) -> Result<u64> {
        match &access_key_request.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(access_key.nonce),
            _ => {
                return Err(ClientError::InvalidResponse(
                    "failed to extract current nonce".into(),
                ))
            }
        }
    }
}

pub struct TokenContractNear {}

impl TokenContractNear {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl TokenContract for TokenContractNear {
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

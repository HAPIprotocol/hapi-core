use async_trait::async_trait;
use near_crypto::SecretKey;
use near_jsonrpc_client::{
    methods::{self, query::RpcQueryRequest},
    JsonRpcClient,
};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::{
    transaction::Transaction,
    transaction::{Action, FunctionCallAction},
    types::{AccountId, BlockReference, Finality, FunctionArgs},
    views::QueryRequest,
};
use serde_json::{from_slice, json};

use super::client::wait_tx_execution;
use crate::{
    client::{
        near::GAS_FOR_TX,
        result::{ClientError, Result, Tx},
        token::TokenContract,
    },
    Amount, HapiCoreOptions,
};

pub struct TokenContractNear {
    client: JsonRpcClient,
    contract_address: AccountId,
    signer: Option<String>,
}

impl TokenContractNear {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        Ok(Self {
            client: JsonRpcClient::connect(options.provider_url),
            contract_address: options.contract_address.try_into()?,
            signer: options.private_key,
        })
    }
}

#[async_trait]
impl TokenContract for TokenContractNear {
    fn is_approve_needed(&self) -> bool {
        false
    }

    async fn transfer(&self, to: &str, amount: Amount) -> Result<Tx> {
        let signer_secret_key = self.signer.as_ref().ok_or(ClientError::SignerError)?;
        let signer_secret_key: SecretKey = signer_secret_key
            .parse()
            .map_err(|_| ClientError::SignerError)?;
        let signer = near_crypto::InMemorySigner::from_secret_key(
            self.contract_address.clone(),
            signer_secret_key.clone(),
        );

        let access_key_query_response = self
            .client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::latest(),
                request: near_primitives::views::QueryRequest::ViewAccessKey {
                    account_id: signer.account_id.clone(),
                    public_key: signer.public_key.clone(),
                },
            })
            .await?;

        let nonce = match &access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => Ok(access_key.nonce),
            _ => Err(ClientError::InvalidResponse(
                "failed to extract current nonce".into(),
            )),
        }?;

        let transaction = Transaction {
            signer_id: signer.account_id.clone(),
            public_key: signer.public_key.clone(),
            nonce: nonce + 1,
            receiver_id: self.contract_address.clone(),
            block_hash: access_key_query_response.block_hash,
            actions: vec![Action::FunctionCall(FunctionCallAction {
                method_name: "ft_transfer_call".to_string(),
                args: json!({"receiver_id": to, "amount": amount})
                    .to_string()
                    .into_bytes(),
                gas: GAS_FOR_TX,
                deposit: 1, // one yoctoNear is required by contract
            })],
        };

        Ok(wait_tx_execution(transaction, signer, &self.client).await?)
    }

    async fn approve(&self, _spender: &str, _amount: Amount) -> Result<Tx> {
        unimplemented!("`approve` is not implemented for Near");
    }

    async fn balance(&self, addr: &str) -> Result<Amount> {
        let request = RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: self.contract_address.parse().unwrap(),
                method_name: "ft_balance_of".to_string(),
                args: FunctionArgs::from(json!({ "account_id" : addr }).to_string().into_bytes()),
            },
        };

        let result = self.client.call(request).await?;
        if let QueryResponseKind::CallResult(result) = result.kind {
            Ok(from_slice::<Amount>(&result.result)?)
        } else {
            Err(ClientError::InvalidResponse(
                "failed to receive call result".into(),
            ))
        }
    }
}

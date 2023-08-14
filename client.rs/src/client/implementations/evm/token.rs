use async_trait::async_trait;
use ethers::{
    prelude::abigen,
    signers::{LocalWallet, Signer as EthersSigner},
    types::Address as EthAddress,
};
use std::{str::FromStr, sync::Arc};

use crate::{
    client::{
        interface::HapiCoreOptions,
        result::{ClientError, Result, Tx},
        token::TokenContract,
    },
    Amount,
};

use super::{
    client::{Provider, Signer},
    error::map_ethers_error,
};

use super::client::LOCAL_CHAIN_ID;

abigen!(
    TOKEN_CONTRACT,
    "../evm/artifacts/contracts/Token.sol/Token.json"
);

pub struct TokenContractEvm {
    pub contract: TOKEN_CONTRACT<Signer>,
}

impl TokenContractEvm {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let provider = Provider::try_from(options.provider_url.as_str())
            .map_err(|e| ClientError::UrlParseError(format!("`provider_url`: {e}")))?;

        let signer = LocalWallet::from_str(options.private_key.unwrap_or_default().as_str())
            .map_err(|e| ClientError::Ethers(format!("`private_key`: {e}")))?
            .with_chain_id(options.chain_id.unwrap_or(LOCAL_CHAIN_ID));

        let client = Signer::new(provider, signer);

        let client = Arc::new(client);

        let token_contract: EthAddress = options
            .contract_address
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`token_contract``: {e}")))?;

        let contract: TOKEN_CONTRACT<Signer> = TOKEN_CONTRACT::new(token_contract, client);

        Ok(Self { contract })
    }
}

#[async_trait]
impl TokenContract for TokenContractEvm {
    fn is_approve_needed(&self) -> bool {
        true
    }

    async fn transfer(&self, to: &str, amount: Amount) -> Result<Tx> {
        let to: EthAddress = to
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`to`: {e}")))?;

        self.contract
            .transfer(to, amount.into())
            .send()
            .await
            .map_err(|e| ClientError::Ethers(format!("`transfer` failed: {e}")))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`transfer` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn approve(&self, spender: &str, amount: Amount) -> Result<Tx> {
        let spender: EthAddress = spender
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`spender`: {e}")))?;

        self.contract
            .approve(spender, amount.into())
            .send()
            .await
            .map_err(|e| ClientError::Ethers(format!("`transfer` failed: {e}")))?
            .await?
            .map_or_else(
                || {
                    Err(ClientError::Ethers(
                        "`transfer` failed: no receipt".to_string(),
                    ))
                },
                |receipt| {
                    Ok(Tx {
                        hash: format!("{:?}", receipt.transaction_hash),
                    })
                },
            )
    }

    async fn balance(&self, addr: &str) -> Result<Amount> {
        let addr: EthAddress = addr
            .parse()
            .map_err(|e| ClientError::EthAddressParse(format!("`addr`: {e}")))?;

        self.contract
            .balance_of(addr)
            .call()
            .await
            .map_err(|e| map_ethers_error("balance", e))
            .map(|a| Amount::from_str(&a.to_string()).unwrap_or_default())
    }
}

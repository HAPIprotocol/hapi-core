use anchor_client::{
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
};

use async_trait::async_trait;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{self, instruction::transfer};
use std::str::FromStr;

use crate::{
    client::{
        interface::HapiCoreOptions,
        result::{ClientError, Result, Tx},
        token::TokenContract,
    },
    Amount,
};

use super::utils::get_signer;

pub struct TokenContractSolana {
    cli: RpcClient,
    signer: Keypair,
    mint: Pubkey,
}

impl TokenContractSolana {
    pub fn new(options: HapiCoreOptions) -> Result<Self> {
        let signer = get_signer(options.private_key)?;
        let cli = RpcClient::new(options.provider_url);
        let mint = Pubkey::from_str(&options.contract_address)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`addr`: {e}")))?;

        Ok(Self { cli, signer, mint })
    }
}

#[async_trait]
impl TokenContract for TokenContractSolana {
    fn is_approve_needed(&self) -> bool {
        false
    }

    async fn transfer(&self, to: &str, amount: Amount) -> Result<Tx> {
        let from_pubkey = self.signer.pubkey();
        let to_pubkey = Pubkey::from_str(to)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`addr`: {e}")))?;

        let to_ata = get_associated_token_address(&to_pubkey, &self.mint);
        let from_ata = get_associated_token_address(&from_pubkey, &self.mint);

        let transfer_instruction = transfer(
            &spl_token::id(),
            &from_ata,
            &to_ata,
            &from_pubkey,
            &[&from_pubkey],
            amount.into(),
        )?;

        let recent_blockhash = self.cli.get_latest_blockhash().await?;

        let transfer_tx = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&from_pubkey),
            &[&self.signer],
            recent_blockhash,
        );

        let hash = self
            .cli
            .send_and_confirm_transaction_with_spinner(&transfer_tx)
            .await?
            .to_string();

        Ok(Tx { hash })
    }

    async fn approve(&self, _spender: &str, _amount: Amount) -> Result<Tx> {
        unimplemented!("`approve` is not implemented for Solana");
    }

    async fn balance(&self, addr: &str) -> Result<Amount> {
        let owner = Pubkey::from_str(addr)
            .map_err(|e| ClientError::SolanaAddressParseError(format!("`addr`: {e}")))?;
        let ata = get_associated_token_address(&owner, &self.mint);

        let balance = self.cli.get_token_account_balance(&ata).await?.amount;

        let res = Amount::from_str(balance.as_str())
            .map_err(|e| ClientError::FailedToParseBalance(e.to_string()))?;

        Ok(res)
    }
}

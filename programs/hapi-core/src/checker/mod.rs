mod address_data;

use anchor_lang::prelude::*;
use std::str::FromStr;

use super::state::address::Category;
use crate::error::ErrorCode;
use address_data::AddressData;

const MAINNET_SOLANA_NETWORK: &str = "GTBRKbzBtqDTvbBDmzAHRmUifyHqFGACgUxFrGHQgq4S";
const DEVNET_SOLANA_NETWORK: &str = "GqZKWhUe7ymmZRxTY3LzW19HzT5cDEQE7m3zdtNW6MsD";

pub enum HapiEnvironment {
    Devnet,
    Mainnet,
}

pub struct HapiChecker {
    program_id: Pubkey,
    solana_network: Pubkey,
    max_risk: u8,
    ignored_categories: Vec<Category>,
}

impl HapiChecker {
    pub fn new(environment: HapiEnvironment) -> Self {
        let (program_id, solana_network) = match environment {
            HapiEnvironment::Devnet => (
                crate::id(),
                Pubkey::from_str(DEVNET_SOLANA_NETWORK).unwrap(),
            ),
            HapiEnvironment::Mainnet => (
                crate::id(),
                Pubkey::from_str(MAINNET_SOLANA_NETWORK).unwrap(),
            ),
        };

        Self {
            program_id,
            solana_network,
            max_risk: 0,
            ignored_categories: Vec::new(),
        }
    }

    pub fn max_risk(&mut self, risk: u8) -> &mut Self {
        self.max_risk = if risk > 10 { 10 } else { risk };
        self
    }

    pub fn ignore_category(&mut self, category: Category) -> &mut Self {
        if self.ignored_categories.iter().position(|c| c == &category) == None {
            self.ignored_categories.push(category);
        }
        self
    }

    pub fn get_hapi_address_seeds<'a>(&'a self, account: &'a Pubkey) -> [&'a [u8]; 4] {
        [
            b"address",
            self.solana_network.as_ref(),
            account.as_ref(),
            &[0u8; 32],
        ]
    }

    pub fn get_hapi_address(&self, account: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.get_hapi_address_seeds(account), &self.program_id)
    }

    pub fn get_account_data<'a>(data: &'a [u8], index: usize) -> u8 {
        *data.get(index).unwrap()
    }

    pub fn check_address_risk(
        &self,
        address_info: &AccountInfo,
        payer_account: &Pubkey,
    ) -> Result<()> {
        let (address_account, address_bump) = self.get_hapi_address(payer_account);

        if address_account != address_info.key() {
            return Err(ErrorCode::UnexpectedAccount.into());
        }

        if address_info.owner.ne(&self.program_id) {
            return Err(ErrorCode::IllegalOwner.into());
        }

        if address_info.data_is_empty() {
            return Ok(());
        }

        let data = AddressData::from(address_info);

        if let Ok(data) = data {
            if address_bump != data.bump {
                return Err(ErrorCode::UnexpectedAccount.into());
            }

            if self.ignored_categories.iter().any(|i| i == &data.category) {
                return Ok(());
            }

            if data.risk > self.max_risk {
                return Err(ErrorCode::HighAccountRisk.into());
            }
        }

        Ok(())
    }
}

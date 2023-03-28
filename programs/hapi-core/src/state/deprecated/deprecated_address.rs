use crate::{
    error::{print_error, ErrorCode},
    id,
    state::address::{Address, Category},
};

use anchor_lang::prelude::*;

pub trait CheckDeprecated {
    fn check(&self, pk: &Pubkey, network: &Pubkey, case_id: u64) -> Result<()>;
}
pub trait DeprecatedAddress: Into<Address> + CheckDeprecated {}

pub fn get_deprecated_address(
    version: u8,
    account_data: &mut &[u8],
) -> Result<impl DeprecatedAddress> {
    match version {
        1 => Ok(AddressV1::try_deserialize_unchecked(account_data)?),
        _ => Err(ErrorCode::InvalidAccountVersion.into()),
    }
}

#[account]
pub struct AddressV1 {
    pub community: Pubkey,
    pub network: Pubkey,
    pub address: [u8; 64],
    pub bump: u8,
    pub case_id: u64,
    pub reporter: Pubkey,
    pub category: Category,
    pub risk: u8,
    pub confirmations: u8,
}

impl Into<Address> for AddressV1 {
    fn into(self) -> Address {
        Address {
            community: self.community,
            network: self.network,
            address: self.address,
            bump: self.bump,
            case_id: self.case_id,
            reporter: self.reporter,
            category: self.category,
            risk: self.risk,
            confirmations: self.confirmations,
            replication_bounty: 0,
        }
    }
}

impl CheckDeprecated for AddressV1 {
    fn check(&self, pk: &Pubkey, network: &Pubkey, case_id: u64) -> Result<()> {
        let (pda_pk, pda_bump) = Pubkey::find_program_address(
            &[
                b"address".as_ref(),
                network.as_ref(),
                self.address[0..32].as_ref(),
                self.address[32..64].as_ref(),
            ],
            &id(),
        );

        if *pk != pda_pk || self.bump != pda_bump {
            return print_error(ErrorCode::UnexpectedAccount);
        }
        if self.case_id != case_id {
            return print_error(ErrorCode::CaseMismatch);
        }
        if self.network != *network {
            return print_error(ErrorCode::NetworkMismatch);
        }

        Ok(())
    }
}

impl DeprecatedAddress for AddressV1 {}

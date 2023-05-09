use crate::{
    error::ErrorCode,
    state::address::{Address, Category},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Address {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Address> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let address: Address = match Address::VERSION {
            // Warning! V0 migration can be performed only once
            1 => AddressV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(address)
    }
}

#[account]
pub struct AddressV0 {
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

impl TryInto<Address> for AddressV0 {
    type Error = Error;
    fn try_into(self) -> Result<Address> {
        Ok(Address {
            version: Address::VERSION,
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
        })
    }
}

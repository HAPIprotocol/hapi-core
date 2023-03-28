use crate::{
    error::ErrorCode,
    state::{address::Category, asset::Asset},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Asset {
    pub fn from_deprecated(version: u8, account_data: &mut &[u8]) -> Result<Asset> {
        let address: Asset = match version {
            1 => AssetV1::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(address)
    }
}

#[account]
pub struct AssetV1 {
    pub community: Pubkey,
    pub network: Pubkey,
    pub mint: [u8; 64],
    pub asset_id: [u8; 32],
    pub bump: u8,
    pub case_id: u64,
    pub reporter: Pubkey,
    pub category: Category,
    pub risk: u8,
    pub confirmations: u8,
}

impl TryInto<Asset> for AssetV1 {
    type Error = Error;
    fn try_into(self) -> Result<Asset> {
        Ok(Asset {
            community: self.community,
            network: self.network,
            mint: self.mint,
            asset_id: self.asset_id,
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

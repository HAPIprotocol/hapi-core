use crate::{
    error::ErrorCode,
    state::{address::Category, asset::Asset},
};
use {anchor_lang::prelude::*, std::convert::TryInto};

impl Asset {
    pub fn from_deprecated(account_data: &mut &[u8]) -> Result<Asset> {
        // TODO: current account version must be less than deprecated account version (exept V0)
        let asset: Asset = match Asset::VERSION {
            // Warning! V0 migration can be performed only once
            1 => AssetV0::try_deserialize_unchecked(account_data)?,
            _ => return Err(ErrorCode::InvalidAccountVersion.into()),
        }
        .try_into()?;

        Ok(asset)
    }
}

#[account]
pub struct AssetV0 {
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

impl TryInto<Asset> for AssetV0 {
    type Error = Error;
    fn try_into(self) -> Result<Asset> {
        Ok(Asset {
            version: Asset::VERSION,
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

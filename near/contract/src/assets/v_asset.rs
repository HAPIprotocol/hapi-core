use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use super::Asset;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VAsset {
    Current(Asset),
}

impl From<Asset> for VAsset {
    fn from(asset: Asset) -> Self {
        VAsset::Current(asset)
    }
}

impl From<VAsset> for Asset {
    fn from(v_asset: VAsset) -> Self {
        match v_asset {
            VAsset::Current(asset) => asset,
        }
    }
}

use ethers::types::U256;
use std::str::FromStr;

use super::{Category, Uuid};

#[derive(Default, Clone, Debug)]
pub struct AssetId(U256);

impl From<U256> for AssetId {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl From<u64> for AssetId {
    fn from(value: u64) -> Self {
        Self(value.into())
    }
}

impl From<AssetId> for U256 {
    fn from(value: AssetId) -> Self {
        value.0
    }
}

impl From<AssetId> for u64 {
    fn from(value: AssetId) -> Self {
        value.0.as_u64()
    }
}

impl FromStr for AssetId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(U256::from_dec_str(s)?))
    }
}

pub struct CreateAssetInput {}
pub struct UpdateAssetInput {}

#[derive(Default, Clone, Debug)]
pub struct Asset {
    pub address: String,
    pub asset_id: AssetId,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

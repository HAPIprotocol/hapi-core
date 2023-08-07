use ethers::types::U256;
use serde::{de, Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::category::Category;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AssetId(U256);

impl Serialize for AssetId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AssetId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        U256::from_dec_str(&s)
            .map(AssetId)
            .map_err(de::Error::custom)
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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

pub struct CreateAssetInput {
    pub address: String,
    pub asset_id: AssetId,
    pub case_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

pub struct UpdateAssetInput {
    pub address: String,
    pub asset_id: AssetId,
    pub case_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Asset {
    pub address: String,
    pub asset_id: AssetId,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

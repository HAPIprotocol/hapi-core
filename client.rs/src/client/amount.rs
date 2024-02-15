use ethers::types::U256;
use near_sdk::json_types::U128;
use serde::{de, Deserialize, Serialize};
use std::str::FromStr;

#[derive(Default, Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct Amount(U256);

impl Amount {
    pub fn normalize_to_u64(&self, decimals: usize) -> u64 {
        let unit: U256 = U256::exp10(decimals);

        (self.0 / unit).as_u64()
    }
}

impl Serialize for Amount {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        U256::from_dec_str(&s)
            .map(Amount)
            .map_err(de::Error::custom)
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<U256> for Amount {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self(value.into())
    }
}

impl From<U128> for Amount {
    fn from(value: U128) -> Self {
        Self(value.0.into())
    }
}

impl From<Amount> for U256 {
    fn from(value: Amount) -> Self {
        value.0
    }
}

impl From<Amount> for U128 {
    fn from(value: Amount) -> Self {
        U128(value.0.as_u128())
    }
}

impl From<Amount> for u64 {
    fn from(value: Amount) -> Self {
        value.0.as_u64()
    }
}

impl FromStr for Amount {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(U256::from_dec_str(s)?))
    }
}

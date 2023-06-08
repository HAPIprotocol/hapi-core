use ethers::types::U256;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Amount(U256);

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

impl From<Amount> for U256 {
    fn from(value: Amount) -> Self {
        value.0
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

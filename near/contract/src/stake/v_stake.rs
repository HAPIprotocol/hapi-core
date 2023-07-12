use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::StakeConfiguration;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VStakeConfiguration {
    Current(StakeConfiguration),
}

impl From<VStakeConfiguration> for StakeConfiguration {
    fn from(v: VStakeConfiguration) -> Self {
        match v {
            VStakeConfiguration::Current(v) => v,
        }
    }
}

impl From<StakeConfiguration> for VStakeConfiguration {
    fn from(v: StakeConfiguration) -> Self {
        VStakeConfiguration::Current(v)
    }
}

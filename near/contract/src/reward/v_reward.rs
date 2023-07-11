use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::RewardConfiguration;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VRewardConfiguration {
    Current(RewardConfiguration),
}

impl From<VRewardConfiguration> for RewardConfiguration {
    fn from(v: VRewardConfiguration) -> Self {
        match v {
            VRewardConfiguration::Current(v) => v,
        }
    }
}

impl From<RewardConfiguration> for VRewardConfiguration {
    fn from(v: RewardConfiguration) -> Self {
        VRewardConfiguration::Current(v)
    }
}

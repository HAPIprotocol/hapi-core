use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    Timestamp,
};

use workspaces::AccountId;

use crate::context::TestContext;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeConfiguration {
    pub token: AccountId,
    pub unlock_duration: Timestamp,
    pub validator_stake: U128,
    pub tracer_stake: U128,
    pub publisher_stake: U128,
    pub authority_stake: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardConfiguration {
    pub token: AccountId,
    pub address_confirmation_reward: U128,
    pub address_tracer_reward: U128,
    pub asset_confirmation_reward: U128,
    pub asset_tracer_reward: U128,
}

pub const UNLOCK_DURATION: u64 = 60; // in seconds
pub const VALIDATOR_STAKE: u128 = 30;
pub const TRACER_STAKE: u128 = 20;
pub const PUBLISHER_STAKE: u128 = 10;
pub const AUTHORITY_STAKE: u128 = 50;

impl TestContext {
    pub async fn get_stake_configuration(&self) -> StakeConfiguration {
        StakeConfiguration {
            token: self.stake_token.id.clone(),
            unlock_duration: UNLOCK_DURATION,
            validator_stake: U128(30),
            tracer_stake: U128(20),
            publisher_stake: U128(10),
            authority_stake: U128(50),
        }
    }

    pub async fn get_reward_configuration(&self) -> RewardConfiguration {
        RewardConfiguration {
            token: self.reward_token.id.clone(),
            address_confirmation_reward: 1.into(),
            address_tracer_reward: 2.into(),
            asset_confirmation_reward: 3.into(),
            asset_tracer_reward: 4.into(),
        }
    }
}

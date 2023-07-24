use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    serde_json::json,
    Timestamp,
};

use workspaces::AccountId;

use crate::context::TestContext;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeAmounts {
    pub validator: U128,
    pub tracer: U128,
    pub publisher: U128,
    pub authority: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeConfiguration {
    pub token: AccountId,
    pub unlock_duration: Timestamp,
    pub stake_amounts: StakeAmounts,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardAmounts {
    pub address_confirmation: U128,
    pub address_trace: U128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardConfiguration {
    pub token: AccountId,
    pub reward_amounts: RewardAmounts,
}

pub const UNLOCK_DURATION: u64 = 60; // in seconds
pub const STAKE_AMOUNTS: StakeAmounts = StakeAmounts {
    validator: U128(30),
    tracer: U128(20),
    publisher: U128(10),
    authority: U128(50),
};

impl TestContext {
    pub async fn get_stake_configuration(&self) -> StakeConfiguration {
        StakeConfiguration {
            token: self.stake_token.id.clone(),
            unlock_duration: UNLOCK_DURATION,
            stake_amounts: STAKE_AMOUNTS,
        }
    }

    pub async fn get_reward_configuration(&self) -> RewardConfiguration {
        json!({
            "reward_configuration": {
                "token": self.reward_token.id,
                "reward_amounts": {
                    "address_confirmation": "1",
                    "address_trace": "2"
                }
            }
        });
        RewardConfiguration {
            token: self.reward_token.id.clone(),
            reward_amounts: RewardAmounts {
                address_confirmation: 1.into(),
                address_trace: 2.into(),
            },
        }
    }
}

use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    serde_json::json,
    Timestamp,
};

use workspaces::AccountId;

use crate::TestContext;

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

impl TestContext {
    pub async fn get_stake_configuration(&self) -> StakeConfiguration {
        json!({
            "stake_configuration": {
                "token": self.stake_token.id,
                "unlock_duration": 60,
                "stake_amounts": {
                    "validator": "30",
                    "tracer": "20",
                    "publisher": "10",
                    "authority": "50"
                }
            }
        });
        StakeConfiguration {
            token: self.stake_token.id.clone(),
            unlock_duration: 60,
            stake_amounts: StakeAmounts {
                validator: 30.into(),
                tracer: 20.into(),
                publisher: 10.into(),
                authority: 50.into(),
            },
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

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    serde::{Deserialize, Serialize},
    AccountId, Balance,
};

mod v_reward;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardAmounts {
    address_confirmation: Balance,
    address_trace: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardConfiguration {
    token: AccountId,
    reward_amounts: RewardAmounts,
}

impl Default for RewardConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            reward_amounts: RewardAmounts {
                address_confirmation: Balance::default(),
                address_trace: Balance::default(),
            },
        }
    }
}

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardAmounts {
    address_confirmation: U128,
    address_trace: U128,
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
                address_confirmation: U128(0),
                address_trace: U128(0),
            },
        }
    }
}

impl RewardConfiguration {
    pub fn get_confirmation_reward(&self) -> U128 {
        self.reward_amounts.address_confirmation
    }

    pub fn get_trace_reward(&self) -> U128 {
        self.reward_amounts.address_trace
    }

    pub fn get_token(&self) -> AccountId {
        self.token.clone()
    }
}

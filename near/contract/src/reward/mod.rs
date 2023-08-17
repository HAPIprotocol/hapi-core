use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardConfiguration {
    token: AccountId,
    address_confirmation_reward: U128,
    tracer_reward: U128,
}

impl Default for RewardConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            address_confirmation_reward: U128(0),
            tracer_reward: U128(0),
        }
    }
}

impl RewardConfiguration {
    pub fn get_confirmation_reward(&self) -> U128 {
        self.address_confirmation_reward
    }

    pub fn get_trace_reward(&self) -> U128 {
        self.tracer_reward
    }

    pub fn get_token(&self) -> &AccountId {
        &self.token
    }

    pub fn is_default(&self) -> bool {
        self.token == env::current_account_id()
    }
}

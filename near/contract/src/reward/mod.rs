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
    address_tracer_reward: U128,
    asset_confirmation_reward: U128,
    asset_tracer_reward: U128,
}

impl Default for RewardConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            address_confirmation_reward: 0.into(),
            address_tracer_reward: 0.into(),
            asset_confirmation_reward: 0.into(),
            asset_tracer_reward: 0.into(),
        }
    }
}

impl RewardConfiguration {
    pub fn get_token(&self) -> &AccountId {
        &self.token
    }

    pub fn is_default(&self) -> bool {
        self.token == env::current_account_id()
    }
}

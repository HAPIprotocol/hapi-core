use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, Balance,
};

mod v_reward;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardAmounts {
    address_confirmation: Balance,
    address_trace: Balance,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardConfiguration {
    token: AccountId,
    reward_amounts: RewardAmounts,
}

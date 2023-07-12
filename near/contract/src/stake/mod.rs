use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, Balance, Timestamp,
};

mod v_stake;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakeAmounts {
    validator: Balance,
    tracer: Balance,
    publisher: Balance,
    authority: Balance,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakeConfiguration {
    token: AccountId,
    unlock_duration: Timestamp,
    stake_amounts: StakeAmounts,
}

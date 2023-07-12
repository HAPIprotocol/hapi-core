use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    serde::{Deserialize, Serialize},
    AccountId, Balance, Timestamp,
};

mod v_stake;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeAmounts {
    validator: Balance,
    tracer: Balance,
    publisher: Balance,
    authority: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeConfiguration {
    token: AccountId,
    unlock_duration: Timestamp,
    stake_amounts: StakeAmounts,
}

impl Default for StakeConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            unlock_duration: Timestamp::default(),
            stake_amounts: StakeAmounts {
                validator: Balance::default(),
                tracer: Balance::default(),
                publisher: Balance::default(),
                authority: Balance::default(),
            },
        }
    }
}

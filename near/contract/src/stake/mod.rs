use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId, Timestamp,
};

mod v_stake;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeAmounts {
    validator: U128,
    tracer: U128,
    publisher: U128,
    authority: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeConfiguration {
    /// address of the stake token mint contract
    token: AccountId,
    /// duration of reporter suspension before the stake can be withdrawn, in seconds
    unlock_duration: Timestamp,
    /// stake amounts for respective reporter types
    stake_amounts: StakeAmounts,
}

impl Default for StakeConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            unlock_duration: Timestamp::default(),
            stake_amounts: StakeAmounts {
                validator: U128(0),
                tracer: U128(0),
                publisher: U128(0),
                authority: U128(0),
            },
        }
    }
}

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    require,
    serde::{Deserialize, Serialize},
    AccountId, Timestamp,
};

use crate::{
    reporter::Role, TimestampExtension, ERROR_INVALID_STAKE_AMOUNT, ERROR_INVALID_STAKE_TOKEN,
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeConfiguration {
    /// address of the stake token mint contract
    token: AccountId,
    /// duration of reporter suspension before the stake can be withdrawn, in seconds
    unlock_duration: Timestamp,
    /// stake amounts for respective reporter types
    validator_stake: U128,
    tracer_stake: U128,
    publisher_stake: U128,
    authority_stake: U128,
}

impl Default for StakeConfiguration {
    fn default() -> Self {
        Self {
            token: env::current_account_id(),
            unlock_duration: Timestamp::default(),
            validator_stake: U128(0),
            tracer_stake: U128(0),
            publisher_stake: U128(0),
            authority_stake: U128(0),
        }
    }
}

impl StakeConfiguration {
    //returns the timestamp in seconds when the stake can be withdrawn
    pub fn get_unlock_timestamp(&self) -> Timestamp {
        env::block_timestamp().to_sec() + self.unlock_duration
    }

    // check if the stake amount is enough for the reporter type
    pub fn assert_stake_sufficient(&self, amount: U128, role: &Role) {
        let stake = match role {
            Role::Validator => self.validator_stake.0,
            Role::Tracer => self.tracer_stake.0,
            Role::Publisher => self.publisher_stake.0,
            Role::Authority => self.authority_stake.0,
            Role::Appraiser => 0,
        };
        require!(amount.0 == stake, ERROR_INVALID_STAKE_AMOUNT)
    }

    pub fn assert_token_valid(&self) {
        require!(
            env::predecessor_account_id() == self.token,
            ERROR_INVALID_STAKE_TOKEN
        )
    }

    pub fn get_token(&self) -> &AccountId {
        &self.token
    }

    pub fn is_default(&self) -> bool {
        self.token == env::current_account_id()
    }
}

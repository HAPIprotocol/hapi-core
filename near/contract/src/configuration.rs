use near_sdk::{env, near_bindgen, require, AccountId};

use crate::{Contract, ContractExt, RewardConfiguration, StakeConfiguration, ONLY_AUTHORITY};

#[near_bindgen]
impl Contract {
    pub fn get_stake_configuration(&self) -> StakeConfiguration {
        self.stake_configuration.clone()
    }

    pub fn update_stake_configuration(&mut self, stake_configuration: StakeConfiguration) {
        self.assert_authority();
        self.stake_configuration = stake_configuration;
    }

    pub fn get_reward_configuration(&self) -> RewardConfiguration {
        self.reward_configuration.clone()
    }

    pub fn update_reward_configuration(&mut self, reward_configuration: RewardConfiguration) {
        self.assert_authority();
        self.reward_configuration = reward_configuration;
    }

    pub fn set_authority(&mut self, authority: AccountId) {
        self.assert_authority();
        self.authority = authority;
    }
}

impl Contract {
    fn assert_authority(&self) {
        require!(
            env::predecessor_account_id().ne(&self.authority),
            ONLY_AUTHORITY
        );
    }
}

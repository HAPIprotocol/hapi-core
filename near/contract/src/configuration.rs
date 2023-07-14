use near_sdk::{env, near_bindgen, require, AccountId};

use crate::{
    reward::RewardConfiguration, stake::StakeConfiguration, Contract, ContractExt, ONLY_AUTHORITY,
};

#[near_bindgen]
impl Contract {
    pub fn update_stake_configuration(&mut self, stake_configuration: StakeConfiguration) {
        self.assert_authority();
        self.stake_configuration = stake_configuration;
    }

    pub fn update_reward_configuration(&mut self, reward_configuration: RewardConfiguration) {
        self.assert_authority();
        self.reward_configuration = reward_configuration;
    }

    pub fn set_authority(&mut self, authority: AccountId) {
        self.assert_authority();
        self.authority = authority;
    }

    pub fn get_configuration(&self) -> (StakeConfiguration, RewardConfiguration) {
        (
            self.stake_configuration.clone(),
            self.reward_configuration.clone(),
        )
    }
}

impl Contract {
    pub(crate) fn assert_authority(&self) {
        require!(
            env::predecessor_account_id().eq(&self.authority),
            ONLY_AUTHORITY
        );
    }
}

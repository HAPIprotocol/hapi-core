use near_sdk::{env, near_bindgen, require, AccountId};

use crate::{
    reward::RewardConfiguration, stake::StakeConfiguration, Contract, ContractExt,
    ERROR_CHANGE_TOKEN, ERROR_ONLY_AUTHORITY, ERROR_REWARD_CONFIGURATION_NOT_SET,
    ERROR_STAKE_CONFIGURATION_NOT_SET,
};

#[near_bindgen]
impl Contract {
    pub fn update_stake_configuration(&mut self, stake_configuration: StakeConfiguration) {
        self.assert_authority();
        require!(
            self.stake_configuration.is_default()
                || stake_configuration
                    .get_token()
                    .eq(self.stake_configuration.get_token()),
            ERROR_CHANGE_TOKEN
        );

        self.stake_configuration = stake_configuration;
    }

    pub fn update_reward_configuration(&mut self, reward_configuration: RewardConfiguration) {
        self.assert_authority();
        require!(
            self.reward_configuration.is_default()
                || reward_configuration
                    .get_token()
                    .eq(self.reward_configuration.get_token()),
            ERROR_CHANGE_TOKEN
        );

        self.reward_configuration = reward_configuration;
    }

    pub fn set_authority(&mut self, authority: AccountId) {
        self.assert_authority();
        self.authority = authority;
    }

    pub fn get_stake_configuration(&self) -> StakeConfiguration {
        require!(
            !self.stake_configuration.is_default(),
            ERROR_STAKE_CONFIGURATION_NOT_SET
        );
        self.stake_configuration.clone()
    }

    pub fn get_reward_configuration(&self) -> RewardConfiguration {
        require!(
            !self.reward_configuration.is_default(),
            ERROR_REWARD_CONFIGURATION_NOT_SET
        );

        self.reward_configuration.clone()
    }

    pub fn get_authority(&self) -> AccountId {
        self.authority.clone()
    }
}

impl Contract {
    pub(crate) fn assert_authority(&self) {
        require!(
            env::predecessor_account_id().eq(&self.authority),
            ERROR_ONLY_AUTHORITY
        );
    }
}

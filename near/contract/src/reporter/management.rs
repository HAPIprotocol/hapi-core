use near_sdk::{
    env::{self, panic_str},
    json_types::U128,
    near_bindgen, require, AccountId,
};

use super::{Reporter, ReporterId, ReporterStatus, Role};
use crate::{
    utils::MAX_NAME_LENGTH, Contract, ContractExt, TimestampExtension, ERROR_LONG_NAME,
    ERROR_REPORTER_EXISTS, ERROR_REPORTER_IS_ACTIVE, ERROR_REPORTER_IS_INACTIVE,
    ERROR_REPORTER_NOT_FOUND, ERROR_UNLOCK_DURATION_NOT_PASSED,
};

#[near_bindgen]
impl Contract {
    pub fn create_reporter(
        &mut self,
        id: ReporterId,
        account_id: AccountId,
        name: String,
        role: Role,
        url: String,
    ) {
        self.assert_authority();

        require!(name.len() <= MAX_NAME_LENGTH, ERROR_LONG_NAME);

        require!(self.reporters.get(&id).is_none(), ERROR_REPORTER_EXISTS);
        require!(
            self.reporters_by_account.get(&account_id).is_none(),
            ERROR_REPORTER_EXISTS
        );

        let reporter = Reporter {
            id,
            account_id: account_id.clone(),
            name,
            role,
            status: ReporterStatus::Inactive,
            stake: U128(0),
            url,
            unlock_timestamp: 0,
        };

        self.reporters.insert(&id.clone(), &reporter.into());
        self.reporters_by_account.insert(&account_id, &id);
    }

    pub fn update_reporter(
        &mut self,
        id: ReporterId,
        account_id: AccountId,
        name: String,
        role: Role,
        url: String,
    ) {
        self.assert_authority();

        require!(name.len() <= MAX_NAME_LENGTH, ERROR_LONG_NAME);

        let mut reporter: Reporter = self
            .reporters
            .get(&id)
            .expect(ERROR_REPORTER_NOT_FOUND)
            .into();

        if reporter.account_id != account_id {
            self.reporters_by_account
                .remove(&reporter.account_id)
                .expect(ERROR_REPORTER_NOT_FOUND);

            reporter.account_id = account_id;
            self.reporters_by_account.insert(&reporter.account_id, &id);
        }

        reporter.name = name;
        reporter.role = role;
        reporter.url = url;

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }

    pub fn deactivate_reporter(&mut self) {
        let mut reporter = self.get_reporter_by_account(env::predecessor_account_id());

        require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_timestamp = self.stake_configuration.get_unlock_timestamp();

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }

    pub fn unstake(&mut self) {
        let mut reporter = self.get_reporter_by_account(env::predecessor_account_id());

        match reporter.status {
            ReporterStatus::Inactive => panic_str(ERROR_REPORTER_IS_INACTIVE),
            ReporterStatus::Active => panic_str(ERROR_REPORTER_IS_ACTIVE),
            ReporterStatus::Unstaking => {}
        }
        require!(
            reporter.status == ReporterStatus::Unstaking,
            ERROR_REPORTER_IS_ACTIVE
        );

        require!(
            env::block_timestamp().to_sec() >= reporter.unlock_timestamp,
            ERROR_UNLOCK_DURATION_NOT_PASSED
        );

        // transfer stake to reporter
        self.transfer_stake(
            reporter.account_id.clone(),
            reporter.stake,
            self.stake_configuration.get_token().clone(),
        );

        reporter.status = ReporterStatus::Inactive;
        reporter.stake = U128(0);

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }
}

impl Contract {
    pub fn activate_reporter(&mut self, account_id: AccountId, amount: U128) {
        let mut reporter = self.get_reporter_by_account(account_id);

        require!(!reporter.is_active(), ERROR_REPORTER_IS_ACTIVE);
        self.stake_configuration
            .assert_stake_sufficient(amount, &reporter.role);

        reporter.stake = amount;
        reporter.status = ReporterStatus::Active;

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }
}

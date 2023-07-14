use near_sdk::{env, json_types::U128, near_bindgen, require, AccountId};

use super::{Reporter, ReporterId, ReporterStatus, Role};
use crate::{Contract, ContractExt, REPORTER_EXISTS, REPORTER_IS_ACTIVE, REPORTER_NOT_FOUND};

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

        require!(self.reporters.get(&id).is_none(), REPORTER_EXISTS);
        require!(
            self.reporters_by_account.get(&account_id).is_none(),
            REPORTER_EXISTS
        );

        let reporter = Reporter {
            id: id.clone(),
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

        let mut reporter: Reporter = self.reporters.get(&id).expect(REPORTER_NOT_FOUND).into();
        reporter.account_id = account_id;
        reporter.name = name;
        reporter.role = role;
        reporter.url = url;
        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }

    pub fn deactivate_reporter(&mut self) {
        let mut reporter = self.get_reporter_by_account(env::predecessor_account_id());

        require!(reporter.is_active(), REPORTER_IS_ACTIVE);

        reporter.status = ReporterStatus::Unstaking;
        reporter.unlock_timestamp = self.stake_configuration.get_unlock_timestamp();

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }

    pub fn unstake(&mut self) {
        let mut reporter = self.get_reporter_by_account(env::predecessor_account_id());

        require!(
            reporter.status == ReporterStatus::Unstaking,
            REPORTER_IS_ACTIVE
        );

        reporter.status = ReporterStatus::Inactive;
        reporter.stake = U128(0);

        // transfer stake to reporter
        self.transfer_stake(
            reporter.account_id.clone(),
            reporter.stake,
            self.stake_configuration.get_token(),
        );

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }
}

impl Contract {
    pub fn activate_reporter(&mut self, account_id: AccountId, amount: U128) {
        let mut reporter = self.get_reporter_by_account(account_id);

        require!(!reporter.is_active(), REPORTER_IS_ACTIVE);
        self.stake_configuration
            .assert_stake_sufficient(amount, &reporter.role);

        reporter.stake = amount;
        reporter.status = ReporterStatus::Active;

        self.reporters
            .insert(&reporter.id.clone(), &reporter.into());
    }
}

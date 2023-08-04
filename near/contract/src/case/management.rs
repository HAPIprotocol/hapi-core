use crate::{
    reporter::Role, utils::MAX_NAME_LENGTH, Contract, ContractExt, ERROR_CASE_ALREADY_EXISTS,
    ERROR_CASE_NOT_FOUND, ERROR_INVALID_ROLE, ERROR_LONG_NAME, ERROR_REPORTER_IS_INACTIVE,
};
use near_sdk::{env, near_bindgen, require};

use super::{Case, CaseId, CaseStatus};

#[near_bindgen]
impl Contract {
    pub fn create_case(&mut self, id: CaseId, name: String, url: String) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        require!(name.len() <= MAX_NAME_LENGTH, ERROR_LONG_NAME);

        match reporter.role {
            Role::Publisher | Role::Authority => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => {
                env::panic_str(ERROR_INVALID_ROLE);
            }
        }

        require!(self.cases.get(&id).is_none(), ERROR_CASE_ALREADY_EXISTS);

        let case = Case {
            id,
            name,
            reporter_id: reporter.id,
            status: CaseStatus::Open,
            url,
        };

        self.cases.insert(&id, &case.into());
    }

    pub fn update_case(&mut self, id: CaseId, name: String, status: CaseStatus, url: String) {
        let reporter = self.get_reporter_by_account(env::predecessor_account_id());

        require!(name.len() <= MAX_NAME_LENGTH, ERROR_LONG_NAME);

        match reporter.role {
            Role::Publisher | Role::Authority => {
                require!(reporter.is_active(), ERROR_REPORTER_IS_INACTIVE);
            }
            _ => {
                env::panic_str(ERROR_INVALID_ROLE);
            }
        }

        let mut case: Case = self.cases.get(&id).expect(ERROR_CASE_NOT_FOUND).into();

        case.name = name;
        case.status = status;
        case.url = url;

        self.cases.insert(&id, &case.into());
    }
}

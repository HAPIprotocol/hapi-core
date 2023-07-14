use near_sdk::{near_bindgen, AccountId};

use super::{Reporter, ReporterId};
use crate::{Contract, ContractExt, REPORTER_NOT_FOUND};

#[near_bindgen]
impl Contract {
    pub fn get_reporter(&self, id: ReporterId) -> Reporter {
        self.reporters.get(&id).expect(REPORTER_NOT_FOUND).into()
    }

    pub fn get_reporters(&self, take: u64, skip: u64) -> Vec<Reporter> {
        self.reporters
            .iter()
            .skip(skip as _)
            .take(take as _)
            .map(|(_, reporter)| reporter.into())
            .collect()
    }

    pub fn get_reporter_count(&self) -> u64 {
        self.reporters.len()
    }

    pub fn get_reporter_by_account(&self, account_id: AccountId) -> Reporter {
        let id = self
            .reporters_by_account
            .get(&account_id)
            .expect(REPORTER_NOT_FOUND);
        self.reporters.get(&id).expect(REPORTER_NOT_FOUND).into()
    }
}

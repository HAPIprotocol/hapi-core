use super::{Case, CaseId};
use crate::{Contract, ContractExt, ERROR_CASE_NOT_FOUND};
use near_sdk::near_bindgen;

#[near_bindgen]
impl Contract {
    pub fn get_case(&self, id: CaseId) -> Case {
        self.cases.get(&id).expect(ERROR_CASE_NOT_FOUND).into()
    }

    pub fn get_cases(&self, skip: u64, take: u64) -> Vec<Case> {
        self.cases
            .iter()
            .skip(skip as _)
            .take(take as _)
            .map(|(_, case)| case.into())
            .collect()
    }

    pub fn get_case_count(&self) -> u64 {
        self.cases.len()
    }
}

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use super::Case;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VCase {
    Current(Case),
}

impl From<Case> for VCase {
    fn from(case: Case) -> Self {
        VCase::Current(case)
    }
}

impl From<VCase> for Case {
    fn from(v_case: VCase) -> Self {
        match v_case {
            VCase::Current(case) => case,
        }
    }
}

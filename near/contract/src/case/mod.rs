use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::ReporterId;

mod v_case;
pub use v_case::VCase;

pub type CaseId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CaseStatus {
    Closed,
    Open,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Case {
    id: CaseId,
    name: String,
    reporter_id: ReporterId,
    status: CaseStatus,
    url: String,
}

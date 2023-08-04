use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

use crate::{utils::UUID, ReporterId};

mod management;
mod v_case;
mod view;
pub use v_case::VCase;

pub type CaseId = UUID;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum CaseStatus {
    Closed,
    Open,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Case {
    id: CaseId,
    name: String,
    reporter_id: ReporterId,
    status: CaseStatus,
    url: String,
}

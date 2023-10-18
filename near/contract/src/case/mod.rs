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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Case {
    pub id: CaseId,
    pub name: String,
    pub reporter_id: ReporterId,
    pub status: CaseStatus,
    pub url: String,
}

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId, Timestamp,
};

mod management;
mod v_reporter;
mod view;

pub use v_reporter::VReporter;

pub type ReporterId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Validator,
    Tracer,
    Publisher,
    Authority,
    Appraiser,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ReporterStatus {
    Inactive,
    Active,
    Unstaking,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Reporter {
    pub id: ReporterId,
    pub account_id: AccountId,
    pub name: String,
    pub role: Role,
    pub status: ReporterStatus,
    pub stake: U128,
    pub url: String,
    pub unlock_timestamp: Timestamp,
}

impl Reporter {
    pub fn is_active(&self) -> bool {
        self.status == ReporterStatus::Active
    }
}

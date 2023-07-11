use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, Balance, Timestamp,
};

mod v_reporter;

pub type ReporterId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Role {
    Validator,
    Tracer,
    Publisher,
    Authority,
    Appraiser,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum ReporterStatus {
    Inactive,
    Active,
    Unstaking,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Reporter {
    pub id: ReporterId,
    pub account_id: AccountId,
    pub name: String,
    pub role: Role,
    pub status: ReporterStatus,
    pub stake: Balance,
    pub url: String,
    pub unlock_timestamp: Timestamp,
}

use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
    AccountId, Timestamp,
};
pub type ReporterId = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Role {
    Validator,
    Tracer,
    Publisher,
    Authority,
    Appraiser,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ReporterStatus {
    Inactive,
    Active,
    Unstaking,
}

#[derive(Serialize, Deserialize)]
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

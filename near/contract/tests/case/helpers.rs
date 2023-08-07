use crate::reporter::ReporterId;
use near_sdk::{serde::{Deserialize, Serialize}, json_types::U128};

pub type CaseId = U128;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum CaseStatus {
    Closed,
    Open,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Case {
    pub id: CaseId,
    pub name: String,
    pub reporter_id: ReporterId,
    pub status: CaseStatus,
    pub url: String,
}

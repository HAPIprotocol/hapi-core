use crate::reporter::ReporterId;
use near_sdk::serde::{Deserialize, Serialize};

pub type CaseId = String;

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

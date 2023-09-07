use crate::{
    address::{Category, RiskScore},
    case::CaseId,
    reporter::ReporterId,
};
use near_sdk::{
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Asset {
    pub address: AccountId,
    pub id: String,
    pub category: Category,
    pub risk_score: RiskScore,
    pub case_id: CaseId,
    pub reporter_id: ReporterId,
    pub confirmations_count: u64,
}

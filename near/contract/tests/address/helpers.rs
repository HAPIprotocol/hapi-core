use crate::{case::CaseId, reporter::ReporterId};
use hapi_core_near::{Category, RiskScore};
use near_sdk::{
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address {
    pub address: AccountId,
    pub category: Category,
    pub risk_score: RiskScore,
    pub case_id: CaseId,
    pub reporter_id: ReporterId,
    pub confirmations_count: u64,
}

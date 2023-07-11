use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    AccountId,
};

use crate::{CaseId, Category, ReporterId, RiskScore};

mod v_asset;

pub type AssetID = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Asset {
    address: AccountId,
    id: String,
    category: Category,
    risk_score: RiskScore,
    case_id: CaseId,
    reporter_id: ReporterId,
    confirmations: UnorderedSet<AccountId>,
}

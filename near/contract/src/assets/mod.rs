use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    AccountId,
};

use crate::{utils::UUID, CaseId, Category, ReporterId, RiskScore};

mod v_asset;
pub use v_asset::VAsset;

pub type AssetId = UUID;

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

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    json_types::U64,
    AccountId,
};

use crate::{CaseId, Category, ReporterId, RiskScore};

mod management;
mod v_asset;
mod view;

pub use v_asset::VAsset;
pub use view::AssetView;

pub type AssetId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Asset {
    address: AccountId,
    id: U64,
    category: Category,
    risk_score: RiskScore,
    case_id: CaseId,
    reporter_id: ReporterId,
    confirmations: UnorderedSet<ReporterId>,
}

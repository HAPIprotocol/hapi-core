use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    AccountId,
};

use crate::{CaseId, Category, ReporterId, RiskScore};

mod management;
mod v_address;
mod view;
pub use v_address::VAddress;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Address {
    address: AccountId,
    category: Category,
    risk_score: RiskScore,
    case_id: CaseId,
    reporter_id: ReporterId,
    confirmations: UnorderedSet<ReporterId>,
}

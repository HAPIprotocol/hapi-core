use crate::{
    CaseId, Category, Contract, ContractExt, ReporterId, RiskScore, ERROR_ASSET_NOT_FOUND,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId};

use super::{Asset, AssetId, VAsset};

// AssetView struct
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetView {
    pub address: AccountId,
    pub id: U64,
    pub category: Category,
    pub risk_score: RiskScore,
    pub case_id: CaseId,
    pub reporter_id: ReporterId,
    pub confirmations_count: u64,
}

impl From<Asset> for AssetView {
    fn from(asset: Asset) -> Self {
        Self {
            address: asset.address,
            id: asset.id,
            category: asset.category,
            risk_score: asset.risk_score,
            case_id: asset.case_id,
            reporter_id: asset.reporter_id,
            confirmations_count: asset.confirmations.len(),
        }
    }
}

impl From<VAsset> for AssetView {
    fn from(v_asset: VAsset) -> Self {
        let asset: Asset = v_asset.into();
        Self {
            address: asset.address,
            id: asset.id,
            category: asset.category,
            risk_score: asset.risk_score,
            case_id: asset.case_id,
            reporter_id: asset.reporter_id,
            confirmations_count: asset.confirmations.len(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_asset(&self, address: AccountId, id: String) -> AssetView {
        let id: AssetId = format!("{}:{}", address, id);
        self.assets.get(&id).expect(ERROR_ASSET_NOT_FOUND).into()
    }

    pub fn get_assets(&self, take: u64, skip: u64) -> Vec<AssetView> {
        self.assets
            .iter()
            .skip(skip as _)
            .take(take as _)
            .map(|(_, asset)| asset.into())
            .collect()
    }

    pub fn get_asset_count(&self) -> u64 {
        self.assets.len()
    }
}

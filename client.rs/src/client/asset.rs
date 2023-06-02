use super::{Category, Uuid};

pub type AssetId = [u8; 32];

pub struct CreateAssetInput {}
pub struct UpdateAssetInput {}

#[derive(Default, Clone)]
pub struct Asset {
    pub address: String,
    pub asset_id: AssetId,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
}

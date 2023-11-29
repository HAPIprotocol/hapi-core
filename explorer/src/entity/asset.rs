use super::types::Category;
use {
    hapi_core::client::entities::asset::Asset as AssetPayload,
    sea_orm::{entity::prelude::*, Set},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "asset")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub address: String,
    pub asset_id: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: u8,
    pub category: Category,
    pub confirmations: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<AssetPayload> for ActiveModel {
    fn from(payload: AssetPayload) -> Self {
        let id = format!("{}.{}", payload.address, payload.asset_id);
        Self {
            id: Set(id),
            address: Set(payload.address.to_owned()),
            asset_id: Set(payload.asset_id.to_string()),
            case_id: Set(payload.case_id.to_owned()),
            reporter_id: Set(payload.reporter_id.to_owned()),
            risk: Set(payload.risk.to_owned()),
            category: Set(payload.category.into()),
            confirmations: Set(payload.confirmations.to_owned()),
        }
    }
}

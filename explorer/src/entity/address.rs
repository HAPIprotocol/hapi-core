use super::{types::Category, FromPayload};
use {
    hapi_core::{client::entities::address::Address as AddressPayload, HapiCoreNetwork},
    sea_orm::{entity::prelude::*, Set},
};

// Risk and confirmations do not correspond to the types of contracts (due to Postgresql restrictions)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: i16,
    pub category: Category,
    pub confirmations: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<AddressPayload> for ActiveModel {
    fn from(network: &HapiCoreNetwork, payload: AddressPayload) -> Self {
        let id = format!("{}.{}", network, payload.address);
        Self {
            id: Set(id),
            address: Set(payload.address.to_owned()),
            case_id: Set(payload.case_id.to_owned()),
            reporter_id: Set(payload.reporter_id.to_owned()),
            risk: Set(payload.risk.into()),
            category: Set(payload.category.into()),
            confirmations: Set(payload.confirmations.to_string()),
        }
    }
}

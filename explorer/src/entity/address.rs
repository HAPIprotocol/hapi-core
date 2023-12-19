use super::{reporter, types::Category, FromPayload};
use {
    hapi_core::client::entities::address::Address as AddressPayload,
    sea_orm::{entity::prelude::*, Set},
};

// Risk and confirmations do not correspond to the types of contracts (due to Postgresql restrictions)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub address: String,
    pub case_id: Uuid,
    pub reporter_id: Uuid,
    pub risk: i16,
    pub category: Category,
    pub confirmations: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "reporter::Entity",
        from = "Column::ReporterId",
        to = "reporter::Column::ReporterId"
    )]
    Reporter,
}

impl Related<reporter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<AddressPayload> for ActiveModel {
    fn from(network_id: Uuid, payload: AddressPayload) -> Self {
        Self {
            network: Set(network_id),
            address: Set(payload.address.to_owned()),
            case_id: Set(payload.case_id.to_owned()),
            reporter_id: Set(payload.reporter_id.to_owned()),
            risk: Set(payload.risk.into()),
            category: Set(payload.category.into()),
            confirmations: Set(payload.confirmations.to_string()),
        }
    }
}

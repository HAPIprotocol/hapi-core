use super::{
    types::{CaseStatus, Network},
    FromPayload,
};

use {
    hapi_core::{client::entities::case::Case as CasePayload, HapiCoreNetwork},
    sea_orm::{entity::prelude::*, Set},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: Network,
    #[sea_orm(primary_key, auto_increment = false)]
    pub case_id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
    pub reporter_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<CasePayload> for ActiveModel {
    fn from(network: &HapiCoreNetwork, payload: CasePayload) -> Self {
        Self {
            network: Set(network.clone().into()),
            case_id: Set(payload.id.to_owned()),
            name: Set(payload.name.to_owned()),
            url: Set(payload.url.to_owned()),
            status: Set(payload.status.into()),
            reporter_id: Set(payload.reporter_id.to_owned()),
        }
    }
}

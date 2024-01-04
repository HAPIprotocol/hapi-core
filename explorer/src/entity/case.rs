use super::{
    types::{CaseStatus, NetworkName},
    FromPayload,
};

use {
    hapi_core::client::entities::case::Case as CasePayload,
    sea_orm::{entity::prelude::*, NotSet, Set},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: NetworkName,
    #[sea_orm(primary_key, auto_increment = false)]
    pub case_id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
    pub reporter_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<CasePayload> for ActiveModel {
    fn from(
        network: NetworkName,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        payload: CasePayload,
    ) -> Self {
        let created_at = created_at.map_or(NotSet, Set);
        let updated_at = updated_at.map_or(NotSet, Set);

        Self {
            network: Set(network),
            case_id: Set(payload.id.to_owned()),
            name: Set(payload.name.to_owned()),
            url: Set(payload.url.to_owned()),
            status: Set(payload.status.into()),
            reporter_id: Set(payload.reporter_id.to_owned()),
            created_at,
            updated_at,
        }
    }
}

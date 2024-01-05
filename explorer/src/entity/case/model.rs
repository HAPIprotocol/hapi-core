use {
    async_graphql::SimpleObject,
    hapi_core::client::entities::case::Case as CasePayload,
    sea_orm::{entity::prelude::*, NotSet, Set},
};

use super::query_utils::{CaseCondition, CaseFilter};
use crate::entity::{
    reporter,
    types::{CaseStatus, NetworkName},
    EntityFilter, FromPayload,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "Case")]
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

impl EntityFilter for Entity {
    type Filter = CaseFilter;
    type Condition = CaseCondition;

    // Fitlering query
    fn filter(selected: Select<Entity>, filter_options: &CaseFilter) -> Select<Entity> {
        let mut query = selected;

        if let Some(network) = filter_options.network {
            query = query.filter(Column::Network.eq(network));
        }

        if let Some(name) = &filter_options.name {
            query = query.filter(Column::Name.eq(name));
        }

        if let Some(url) = &filter_options.url {
            query = query.filter(Column::Url.eq(url));
        }

        if let Some(status) = filter_options.status {
            query = query.filter(Column::Status.eq(status));
        }

        if let Some(reporter_id) = filter_options.reporter_id {
            query = query.filter(Column::ReporterId.eq(reporter_id));
        }

        query
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "reporter::Entity",
        from = "Column::ReporterId",
        to = "reporter::model::Column::ReporterId"
    )]
    Reporter,
}

impl Related<reporter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

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

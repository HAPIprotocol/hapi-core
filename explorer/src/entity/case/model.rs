use {
    async_graphql::SimpleObject,
    hapi_core::client::entities::case::Case as CasePayload,
    sea_orm::{
        entity::prelude::*, EntityTrait, JoinType, NotSet, QueryOrder, QuerySelect, Select, Set,
    },
};

use super::query_utils::{CaseCondition, CaseFilter};
use crate::entity::{
    address, asset,
    pagination::{order_by_column, Ordering},
    reporter,
    types::CaseStatus,
    EntityFilter, FromPayload,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "Case")]
#[sea_orm(table_name = "case")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network_id: String,
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

    // Filtering query
    fn filter(selected: Select<Entity>, filter_options: &CaseFilter) -> Select<Entity> {
        let mut query = selected;

        if let Some(network) = &filter_options.network_id {
            query = query.filter(Column::NetworkId.eq(network));
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

    // Ordering query
    fn order(
        selected: Select<Entity>,
        ordering: Ordering,
        condition: CaseCondition,
    ) -> Select<Entity> {
        match condition {
            CaseCondition::AddressCount => sort_by_count(selected, ordering, Relation::Address),
            CaseCondition::AssetCount => sort_by_count(selected, ordering, Relation::Asset),
            _ => order_by_column(selected, ordering, condition),
        }
    }
}

fn sort_by_count(
    selected: Select<Entity>,
    ordering: Ordering,
    relation: Relation,
) -> Select<Entity> {
    let query = selected
        .column_as(Expr::cust("COUNT(*)"), "related")
        .join(JoinType::InnerJoin, relation.def())
        .group_by(Column::CaseId)
        .group_by(Column::NetworkId);

    match ordering {
        Ordering::Asc => query.order_by_asc(Expr::cust("related")),
        Ordering::Desc => query.order_by_desc(Expr::cust("related")),
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
    #[sea_orm(has_many = "address::Entity")]
    Address,
    #[sea_orm(has_many = "asset::Entity")]
    Asset,
}

impl Related<reporter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

impl Related<address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<asset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Asset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<CasePayload> for ActiveModel {
    fn from(
        network_id: String,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        payload: CasePayload,
    ) -> Self {
        let created_at = created_at.map_or(NotSet, Set);
        let updated_at = updated_at.map_or(NotSet, Set);

        Self {
            network_id: Set(network_id),
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

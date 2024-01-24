use {
    async_graphql::SimpleObject,
    hapi_core::client::entities::reporter::Reporter as ReporterPayload,
    sea_orm::{entity::prelude::*, NotSet, Set},
};

use super::query_utils::{ReporterCondition, ReporterFilter};
use crate::entity::{
    address, asset, case,
    types::{ReporterRole, ReporterStatus},
    EntityFilter, FromPayload,
};

// Note: unlock_timestamp and stake do not correspond to the types of contracts (due to Postgresql restrictions)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "Reporter")]
#[sea_orm(table_name = "reporter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub network_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub account: String,
    pub role: ReporterRole,
    pub status: ReporterStatus,
    pub name: String,
    pub url: String,
    pub stake: String,
    pub unlock_timestamp: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl EntityFilter for Entity {
    type Filter = ReporterFilter;
    type Condition = ReporterCondition;

    // Filtering query
    fn filter(selected: Select<Entity>, filter_options: &ReporterFilter) -> Select<Entity> {
        let mut query = selected;

        if let Some(network) = &filter_options.network_id {
            query = query.filter(Column::NetworkId.eq(network));
        }

        if let Some(account) = &filter_options.account {
            query = query.filter(Column::Account.eq(account));
        }

        if let Some(role) = filter_options.role {
            query = query.filter(Column::Role.eq(role));
        }

        if let Some(status) = filter_options.status {
            query = query.filter(Column::Status.eq(status));
        }

        if let Some(name) = &filter_options.name {
            query = query.filter(Column::Name.eq(name));
        }

        if let Some(url) = &filter_options.url {
            query = query.filter(Column::Url.eq(url));
        }

        query
    }

    /// Columns for search
    fn columns_for_search() -> Vec<String> {
        vec![
            String::from("network_id"),
            String::from("id::text"),
            String::from("account"),
            String::from("role::text"),
            String::from("status::text"),
            String::from("name"),
            String::from("url"),
            String::from("stake"),
            String::from("unlock_timestamp"),
            String::from("created_at::text"),
            String::from("updated_at::text"),
        ]
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "address::Entity")]
    Address,
    #[sea_orm(has_many = "asset::Entity")]
    Asset,
    #[sea_orm(has_many = "case::Entity")]
    Case,
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

impl Related<case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Case.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl FromPayload<ReporterPayload> for ActiveModel {
    fn from(
        network_id: String,
        created_at: Option<DateTime>,
        updated_at: Option<DateTime>,
        payload: ReporterPayload,
    ) -> Self {
        let created_at = created_at.map_or(NotSet, Set);
        let updated_at = updated_at.map_or(NotSet, Set);

        Self {
            network_id: Set(network_id),
            id: Set(payload.id.to_owned()),
            account: Set(payload.account.to_owned()),
            role: Set(payload.role.into()),
            status: Set(payload.status.into()),
            name: Set(payload.name.to_owned()),
            url: Set(payload.url.to_owned()),
            stake: Set(payload.stake.to_string()),
            unlock_timestamp: Set(payload.unlock_timestamp.to_string()),
            created_at,
            updated_at,
        }
    }
}

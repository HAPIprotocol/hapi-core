use super::query_utils::{NetworkCondition, NetworkFilter};
use crate::entity::{types::NetworkBackend, EntityFilter};

use {async_graphql::SimpleObject, sea_orm::entity::prelude::*};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[graphql(name = "Network")]
#[sea_orm(table_name = "network")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub backend: NetworkBackend,
    pub chain_id: Option<String>,
    pub authority: String,
    pub stake_token: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl EntityFilter for Entity {
    type Filter = NetworkFilter;
    type Condition = NetworkCondition;

    // Filtering query
    fn filter(selected: Select<Entity>, filter_options: &NetworkFilter) -> Select<Entity> {
        let mut query = selected;

        if let Some(name) = &filter_options.name {
            query = query.filter(Column::Name.contains(name));
        }

        if let Some(backend) = filter_options.backend {
            query = query.filter(Column::Backend.eq(backend));
        }

        if let Some(authority) = &filter_options.authority {
            query = query.filter(Column::Authority.eq(authority));
        }

        if let Some(stake_token) = &filter_options.stake_token {
            query = query.filter(Column::StakeToken.eq(stake_token));
        }

        query
    }

    /// Columns for search
    fn columns_for_search() -> Vec<String> {
        vec![
            String::from("id"),
            String::from("name"),
            String::from("backend::text"),
            String::from("chain_id"),
            String::from("authority"),
            String::from("stake_token"),
            String::from("created_at::text"),
            String::from("updated_at::text"),
        ]
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

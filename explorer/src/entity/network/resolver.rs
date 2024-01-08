use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    std::sync::Arc,
    tracing::instrument,
    uuid::Uuid,
};

use super::{
    model::Model,
    query_utils::{NetworkCondition, NetworkFilter},
};

use crate::{
    entity::pagination::{EntityInput, EntityPage},
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct NetworkQuery {}

/// Queries for the `Network` model
#[Object]
impl NetworkQuery {
    /// Get a single address
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_network(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Network id")] id: Uuid,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let address = EntityQuery::find_entity_by_id::<super::model::Entity, _>(db, id).await?;

        Ok(address)
    }

    /// Get multiple addresses
    #[instrument(level = "debug", skip(self, ctx), fields(input = ?input))]
    pub async fn get_many_networkss(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Network input parameters")] input: EntityInput<
            NetworkFilter,
            NetworkCondition,
        >,
    ) -> Result<EntityPage<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

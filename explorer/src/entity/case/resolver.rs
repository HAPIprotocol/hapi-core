use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    tracing::instrument,
    uuid::Uuid,
};

use super::{
    model::Model,
    query_utils::{CaseCondition, CaseFilter},
};

use crate::{
    entity::pagination::{EntityInput, EntityPage},
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct CaseQuery {}

/// Queries for the `Case` model
#[Object]
impl CaseQuery {
    /// Get a single case
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_case(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Case id")] id: Uuid,
        #[graphql(desc = "Case network")] network_id: String,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let address =
            EntityQuery::find_entity_by_id::<super::model::Entity, _>(db, (network_id, id)).await?;

        Ok(address)
    }

    /// Get multiple cases
    #[instrument(level = "debug", skip(self, ctx), fields(input = ?input))]
    pub async fn get_many_cases(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Case input parameters")] input: EntityInput<CaseFilter, CaseCondition>,
    ) -> Result<EntityPage<Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

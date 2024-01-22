use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    tracing::instrument,
    uuid::Uuid,
};

use super::{
    model::Model,
    query_utils::{ReporterCondition, ReporterFilter},
};

use crate::{
    entity::pagination::{EntityInput, EntityPage},
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct ReporterQuery {}

/// Queries for the `Reporter` model
#[Object]
impl ReporterQuery {
    /// Get a single reporter
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_reporter(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Reporter id")] reporter_id: Uuid,
        #[graphql(desc = "Reporter network")] network_id: String,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let address = EntityQuery::find_entity_by_id::<super::model::Entity, _>(
            db,
            (network_id, reporter_id),
        )
        .await?;

        Ok(address)
    }

    /// Get multiple reporters
    #[instrument(level = "debug", skip(self, ctx), fields(input = ?input))]
    pub async fn get_many_reporters(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Reporter input parameters")] input: EntityInput<
            ReporterFilter,
            ReporterCondition,
        >,
    ) -> Result<EntityPage<Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

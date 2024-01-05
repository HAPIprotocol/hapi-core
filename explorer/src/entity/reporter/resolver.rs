use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    std::sync::Arc,
    tracing::instrument,
    uuid::Uuid,
};

use super::{
    model::Model,
    query_utils::{ReporterCondition, ReporterFilter},
};
use crate::{
    entity::{
        pagination::{EntityInput, EntityPage},
        types::NetworkName,
    },
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct ReporterQuery {}

/// Queries for the `Reporter` model
#[Object]
impl ReporterQuery {
    /// Get a single address
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_reporter(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Reporter id")] reporter_id: Uuid,
        #[graphql(desc = "Reporter network")] network: NetworkName,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let address =
            EntityQuery::find_entity_by_id::<super::model::Entity, _>(db, (network, reporter_id))
                .await?;

        Ok(address)
    }

    /// Get multiple addresses
    #[instrument(level = "debug", skip(self, ctx), fields(input = ?input))]
    pub async fn get_many_reporters(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Reporter input parameters")] input: EntityInput<
            ReporterFilter,
            ReporterCondition,
        >,
    ) -> Result<EntityPage<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

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
    entity::{
        pagination::{EntityInput, EntityPage},
        types::NetworkBackend,
    },
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct CaseQuery {}

/// Queries for the `Case` model
#[Object]
impl CaseQuery {
    /// Get a single address
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_case(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Case id")] case_id: Uuid,
        #[graphql(desc = "Case network")] network: NetworkBackend,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let address =
            EntityQuery::find_entity_by_id::<super::model::Entity, _>(db, (network, case_id))
                .await?;

        Ok(address)
    }

    /// Get multiple addresses
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

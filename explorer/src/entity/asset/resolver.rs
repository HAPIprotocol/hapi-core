use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    std::sync::Arc,
    tracing::instrument,
};

use super::{
    model::Model,
    query_utils::{AssetCondition, AssetFilter},
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
pub struct AssetQuery {}

/// Queries for the `Asset` model
#[Object]
impl AssetQuery {
    /// Get a single address
    #[instrument(level = "debug", skip(self, ctx))]
    pub async fn get_asset(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Asset address")] address: String,
        #[graphql(desc = "Asset id")] asset_id: String,
        #[graphql(desc = "Asset network")] network: NetworkName,
    ) -> Result<Option<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let address = EntityQuery::find_entity_by_id::<super::model::Entity, _>(
            db,
            (network, address, asset_id),
        )
        .await?;

        Ok(address)
    }

    /// Get multiple addresses
    #[instrument(level = "debug", skip(self, ctx), fields(input = ?input))]
    pub async fn get_many_assets(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Asset input parameters")] input: EntityInput<AssetFilter, AssetCondition>,
    ) -> Result<EntityPage<Model>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();
        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

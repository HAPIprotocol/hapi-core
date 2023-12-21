use {
    async_graphql::{Context, Object, Result},
    sea_orm::DatabaseConnection,
    std::sync::Arc,
    uuid::Uuid,
};

use super::{
    model,
    query_utils::{AddressCondition, AddressFilter},
};
use crate::{
    entity::pagination::{EntityInput, EntityPage},
    service::EntityQuery,
};

/// The GraphQl Query segment
#[derive(Default)]
pub struct AddressQuery {}

/// Queries for the `Address` model
#[Object]
impl AddressQuery {
    /// Get a single address
    pub async fn get_address(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Address address")] address: String,
        // #[graphql(desc = "Address network")] network: NetworkName,
        #[graphql(desc = "Address network")] network: Uuid,
    ) -> Result<Option<model::Address>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();

        let address =
            EntityQuery::find_entity_by_id::<super::model::Entity, _>(db, (network, address))
                .await?;

        Ok(address)
    }

    /// Get multiple addresses
    pub async fn get_many_addresses(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Address input parameters")] input: EntityInput<
            AddressFilter,
            AddressCondition,
        >,
    ) -> Result<EntityPage<model::Address>> {
        let db = ctx.data_unchecked::<Arc<DatabaseConnection>>();

        let page = EntityQuery::find_many::<super::model::Entity>(db, input).await?;

        Ok(page)
    }
}

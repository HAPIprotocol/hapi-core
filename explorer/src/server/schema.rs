use {
    anyhow::Result,
    async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema},
    sea_orm::DatabaseConnection,
    std::sync::Arc,
};

use crate::entity::address::AddressQuery;

#[derive(Default, MergedObject)]
pub struct Query(AddressQuery);

// #[Object]
// impl Query {
//     /// Get the current User from the GraphQL context
//     async fn versions(&self, ctx: &Context<'_>) -> u8 {
//         5
//     }
// }

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub(crate) fn create_graphql_schema(db: Arc<DatabaseConnection>) -> Result<AppSchema> {
    Ok(
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(db)
            .finish(),
    )
}

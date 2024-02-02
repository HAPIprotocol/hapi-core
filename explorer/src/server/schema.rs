use {
    anyhow::Result,
    async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema},
    sea_orm::DatabaseConnection,
};

use crate::entity::{
    address::AddressQuery, asset::AssetQuery, case::CaseQuery, network::NetworkQuery,
    reporter::ReporterQuery, statistics::StatisticsQuery,
};

/// Top-level application Query type
#[derive(Default, MergedObject)]
pub struct Query(
    AddressQuery,
    AssetQuery,
    CaseQuery,
    ReporterQuery,
    NetworkQuery,
    StatisticsQuery,
);

/// Top-level merged application schema
pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Building the GraphQL application schema, attaching the Database to the context
pub(crate) fn create_graphql_schema(db: DatabaseConnection) -> Result<AppSchema> {
    Ok(
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(db)
            .finish(),
    )
}

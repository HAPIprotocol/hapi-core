use anyhow::Result;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

#[derive(MergedObject, Default)]
pub struct Query();

pub type GraphQLSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub(crate) fn create_schema() -> Result<GraphQLSchema> {
    Ok(Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish())
}

/// Handle GraphiQL Requests
pub(crate) async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// Handle GraphQL Requests
pub(crate) async fn graphql_handler(
    schema: State<GraphQLSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

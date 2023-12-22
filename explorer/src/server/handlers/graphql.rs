use {
    async_graphql_axum::{GraphQLRequest, GraphQLResponse},
    axum::{
        response::{Html, IntoResponse},
        Extension,
    },
};

use crate::server::schema::AppSchema;

// /// Handle GraphiQL Requests
// pub(crate) async fn graphiql() -> impl IntoResponse {
//     println!("I am here 12345");
//     Html(GraphiQLSource::build().endpoint("/graphql").finish())
// }

/// Handle GraphiQL Requests
pub(crate) async fn graphiql() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}

/// Handle GraphQL Requests
pub(crate) async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

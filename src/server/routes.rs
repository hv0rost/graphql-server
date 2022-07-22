use crate::gql_schema;

use warp::{Reply, Rejection, Filter, filters::BoxedFilter, http::Response};
use async_graphql::{Schema, Request, http::playground_source, http::GraphQLPlaygroundConfig};
use std::convert::Infallible;
use async_graphql_warp::GraphQLResponse;

//Check the server is alive
async fn end_route() -> Result<impl Reply, Rejection> {
    Ok(serde_json::to_string({"2 + 2" ; "server is running..."}).unwrap())
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)>{
    //Build the GraphQL gql_schema
    let schema = gql_schema::build_schema().finish();

    let end_route = warp::path::end().and_then(end_route);

    //GraphQL query handler.
    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
        |(schema, request) : (Schema<_,_,_>, Request) | async move{
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        })
    ));

    //GraphQL Playground
    let graphql_playground = warp::path("playground").map(||{
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    //Wire together all the routs.
    end_route
        .or(graphql_handler)
        .or(graphql_playground)
        .boxed()
}
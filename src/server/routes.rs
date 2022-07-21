use crate::gql_schema;

use warp::{Reply, Rejection, Filter, filters::BoxedFilter, http::Response};
use async_graphql::{Schema, Request, http::playground_source, http::GraphQLPlaygroundConfig};
use std::convert::Infallible;
use async_graphql_warp::GraphQLResponse;
use crate::data_base::{connection::establish_connection, db_req::*};

//Check the server is alive
async fn health() -> Result<impl Reply, Rejection> {
    let connection = establish_connection();
    //create_test(&connection, &6, &"12345");
    //println!("{}", serialized);
    delete_test(&connection);
    Ok(serde_json::to_string(&get_all_tests(&connection)).unwrap())
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)>{
    //Build the GraphQL gql_schema
    let schema = gql_schema::build_schema().finish();

    let health = warp::path::end().and_then(health);

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
    health
        .or(graphql_handler)
        .or(graphql_playground)
        .boxed()
}
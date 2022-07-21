mod health;

use async_graphql::{Schema, SchemaBuilder, MergedObject, EmptyMutation, EmptySubscription};

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

//Build the GraphQL gql_schema.
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription >{
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
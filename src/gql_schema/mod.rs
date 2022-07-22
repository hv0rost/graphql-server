mod graphql;
mod graphql_fields;

use async_graphql::{Schema, SchemaBuilder, EmptySubscription};
use crate::gql_schema::graphql::{Mutation, Query};

//Build the GraphQL gql_schema.
pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription >{
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
mod test;

use async_graphql::{Schema, SchemaBuilder, MergedObject, EmptySubscription};
use crate::gql_schema::test::Mutation;

#[derive(MergedObject, Default)]
pub struct Query(test::TestQuery);

/*#[derive(MergedObject, Default)]
pub struct Mutation(test::Mutation);*/

//Build the GraphQL gql_schema.
pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription >{
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
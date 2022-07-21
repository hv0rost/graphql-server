use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use async_graphql::{Context, Object};
use crate::data_base::{db_req, models::*, schema::test};
use crate::data_base::connection::get_conn_from_ctx;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery{
    async fn health(&self, ctx: &Context<'_>) -> Vec<Test>{
        db_req::get_all_tests(&get_conn_from_ctx(ctx))
            .expect("Can't get tests")
            .iter()
            .map(Test::from)
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    id: i32,
    name: String,
}

#[Object]
impl Test {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

impl From<&TestEntity> for Test {
    fn from(test: &TestEntity) -> Self {
        Test {
            id: test.id.into(),
            name: test.name.clone(),
        }
    }
}
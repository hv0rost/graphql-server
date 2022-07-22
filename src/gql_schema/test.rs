use serde::{Deserialize, Serialize};
use async_graphql::{Context, Object};
use crate::data_base::{db_req, models::*};
use crate::data_base::connection::{establish_connection, get_conn_from_ctx};

#[derive(Default)]
pub struct TestQuery;

#[Object]
impl TestQuery{
    async fn get_all_tests(&self) -> Vec<Test>{
        let conn = establish_connection();
        db_req::get_all_tests(&conn)
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
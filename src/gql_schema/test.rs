use serde::{Deserialize, Serialize};
use async_graphql::*;
use crate::data_base::{db_req, models::*};
use crate::data_base::connection::establish_connection;

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

#[derive(Default)]
pub struct Mutation;
#[Object]
impl Mutation {
    async fn create_test(&self, name : String) -> Result<Test> {
        let new_test = NewTestEntity {
            name,
        };

        let created_test_entity =
            db_req::create_test(new_test, &establish_connection())?;

        Ok(Test::from(&created_test_entity))
    }

    async fn delete_test(&self, id : i32) -> Result<Test>{
            let deleted_test_entity =
            db_req::delete_test(id, &establish_connection())?;

        Ok(Test::from(&deleted_test_entity))
    }

    async fn update_test(&self, id : i32, new_name: String) -> Result<Test>{
        let updated_test_entity =
            db_req::update_test(&establish_connection(), id, new_name)?;

        Ok(Test::from(&updated_test_entity))
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
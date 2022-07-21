use serde::Serialize;

#[derive(Queryable, Default, Debug, Serialize)]
pub struct TestEntity {
    pub id: i32,
    pub name: String,
}

use super::schema::test;

#[derive(Insertable)]
#[table_name="test"]
pub struct NewTest<'a> {
    pub id: &'a i32,
    pub name: &'a str,
}
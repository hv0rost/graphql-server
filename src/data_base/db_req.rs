use diesel::prelude::*;
use crate::data_base;
use data_base::models::*;
use crate::data_base::schema::test;

pub fn create_test(new_test : NewTestEntity, conn : &PgConnection) -> QueryResult<TestEntity> {
    let created_test: TestEntity = diesel::insert_into(test::table)
        .values(new_test)
        .get_result(conn)?;
    Ok(created_test)
}

pub fn get_all_tests(conn: &PgConnection) -> QueryResult<Vec<TestEntity>> {
    test::table.load(conn)
}

pub fn delete_test(del_test : i32 ,conn: &PgConnection) -> QueryResult<TestEntity>{
    let deleted_test : TestEntity = diesel::delete(test::table
        .filter(test::id.eq(del_test)))
        .get_result(conn)?;
    Ok(deleted_test)
}

pub fn update_test(conn : &PgConnection, upd_id : i32, new_name : String) -> QueryResult<TestEntity>{
    let updated_test : TestEntity = diesel::update(test::table
        .filter(test::id.eq(upd_id)))
        .set(test::name.eq(new_name))
        .get_result(conn)?;
    Ok(updated_test)
}
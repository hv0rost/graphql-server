use diesel::prelude::*;
use crate::data_base;
use data_base::models::*;
use crate::data_base::schema::test;

pub fn create_test<'a>(conn: &PgConnection, new_id: &'a i32, new_name: &'a str) -> Test {

    let new_test = NewTest {
        id : new_id,
        name : new_name,
    };

    diesel::insert_into(test::table)
        .values(&new_test)
        .get_result(conn)
        .expect("Error saving new tests")
}

pub fn get_all_tests(conn: &PgConnection) -> Vec<Test> {
    test::table
        .limit(5)
        .load::<Test>(conn)
        .expect("Error loading tests")
}

pub fn delete_test(conn: &PgConnection) {
    diesel::delete(test::table.filter(test::id.eq(1)))
        .execute(conn)
        .expect("Error deleting tests");
}
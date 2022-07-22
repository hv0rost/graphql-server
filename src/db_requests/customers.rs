use diesel::prelude::*;
use crate::data_base;
use data_base::models::*;
use crate::data_base::schema::customers;

pub fn get_all_customers(conn: &PgConnection) -> QueryResult<Vec<CustomersEntity>> {
    customers::table.load(conn)
}
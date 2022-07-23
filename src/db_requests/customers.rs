use diesel::prelude::*;
use crate::data_base;
use data_base::models::*;
use crate::data_base::schema::customers;

pub fn get_all_customers(conn: &PgConnection) -> QueryResult<Vec<CustomersEntity>> {
    customers::table.load(conn)
}

pub fn create_customer(new_customer : NewCustomerEntity, conn : &PgConnection) -> QueryResult<CustomersEntity> {
    let created_customer: CustomersEntity = diesel::insert_into(customers::table)
        .values(new_customer)
        .get_result(conn)?;
    Ok(created_customer)
}
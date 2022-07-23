use diesel::prelude::*;
use crate::data_base;
use data_base::models::*;
use crate::data_base::schema::{stores, customers};

pub fn get_all_stores(conn: &PgConnection) -> QueryResult<Vec<StoreEntity>> {
    stores::table.load(conn)
}

pub fn get_stores_from_customer_name(conn: &PgConnection, customers_name : String) -> QueryResult<Vec<StoreEntity>> {
    let id : i32 = customers::table.filter(customers::name.eq(customers_name)).select(customers::id).load(conn)?[0];
    stores::table.filter(stores::clients.eq(id)).load(conn)
}

pub fn create_store(new_store : NewStoreEntity, conn : &PgConnection) -> QueryResult<StoreEntity> {
    let created_store: StoreEntity = diesel::insert_into(stores::table)
        .values(new_store)
        .get_result(conn)?;
    Ok(created_store)
}

pub fn delete_store(del_test : i32 ,conn: &PgConnection) -> QueryResult<StoreEntity>{
    let deleted_store : StoreEntity = diesel::delete(stores::table
        .filter(stores::id.eq(del_test)))
        .get_result(conn)?;
    Ok(deleted_store)
}

pub fn update_store(conn : &PgConnection, upd_id : i32, new_name : String) -> QueryResult<StoreEntity>{
    let updated_store : StoreEntity = diesel::update(stores::table
        .filter(stores::id.eq(upd_id)))
        .set(stores::name.eq(new_name))
        .get_result(conn)?;
    Ok(updated_store)
}

use serde::Serialize;
use super::schema::{stores, customers};

#[derive(Queryable, Identifiable, Default, Debug, Serialize)]
#[table_name="stores"]
pub struct StoreEntity {
    pub id: i32,
    pub name: String,
    pub clients: Option<i32>,
}


#[derive(Insertable)]
#[table_name="stores"]
pub struct NewStoreEntity {
    pub name: String,
    pub clients: Option<i32>,
}

#[derive(Queryable, Identifiable)]
#[table_name="customers"]
pub struct CustomersEntity{
    pub id : i32,
    pub name : String,
    pub data : Option<serde_json::Value>
}

#[derive(Insertable)]
#[table_name="customers"]
pub struct NewCustomerEntity {
    pub name: String,
    pub data: Option<serde_json::Value>,
}
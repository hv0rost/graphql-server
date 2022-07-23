use async_graphql::*;
use crate::data_base::models::*;
use crate::data_base::connection::establish_connection;
use crate::gql_schema::graphql_fields::{Customer, Store};
use crate::db_requests::*;

#[derive(Default)]
pub struct Query;
#[Object]
impl Query {
    //Store field
    async fn get_all_stores(&self) -> Vec<Store> {
        let conn = establish_connection();
        store::get_all_stores(&conn)
            .expect("Can't get stores")
            .iter()
            .map(Store::from)
            .collect()
    }

    async fn get_stores_from_customer_name(&self, customers_name: String) -> Vec<Store> {
        store::get_stores_from_customer_name(&establish_connection(), customers_name)
            .expect("Can't get customers")
            .iter()
            .map(Store::from)
            .collect()
    }

    //Customers field
    async fn get_all_customers(&self) -> Vec<Customer> {
        customers::get_all_customers(&establish_connection())
        .expect("Can't get customers")
        .iter()
        .map(Customer::from)
        .collect()
    }
}
#[derive(Default)]
pub struct Mutation;
#[Object]
impl Mutation {
    async fn create_store(&self, name : String, clients: Option<i32>) -> Result<Store> {
        let new_store = NewStoreEntity {
            name,
            clients,
        };

        let created_store_entity =
            store::create_store(new_store, &establish_connection())?;
        Ok(Store::from(&created_store_entity))
    }

    async fn create_customer(&self, name : String, data: Option<serde_json::Value>) -> Result<Customer> {
        let new_customer = NewCustomerEntity {
            name,
            data,
        };

        let created_customer_entity =
            customers::create_customer(new_customer, &establish_connection())?;
        Ok(Customer::from(&created_customer_entity))
    }

    async fn delete_store(&self, id : i32) -> Result<Store>{
            let deleted_store_entity =
                store::delete_store(id, &establish_connection())?;

        Ok(Store::from(&deleted_store_entity))
    }

    async fn update_store(&self, id : i32, new_name: String) -> Result<Store>{
        let updated_store_entity =
            store::update_store(&establish_connection(), id, new_name)?;

        Ok(Store::from(&updated_store_entity))
    }
}


use serde::{Deserialize, Serialize};
use crate::data_base::models::{StoreEntity, CustomersEntity};
use async_graphql::*;

#[derive(Serialize, Deserialize)]
pub struct Store {
    id: i32,
    name: String,
    clients : Option<i32>,
}

#[Object]
impl Store {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn clients(&self) -> &Option<i32> { &self.clients }
}

impl From<&StoreEntity> for Store {
    fn from(store: &StoreEntity) -> Self {
        Store {
            id: store.id.into(),
            name: store.name.clone(),
            clients: store.clients.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
    id: i32,
    name: String,
}

#[Object]
impl Customer {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
}

impl From<&CustomersEntity> for Customer {
    fn from(customer: &CustomersEntity) -> Self {
        Customer {
            id: customer.id.into(),
            name: customer.name.clone(),
        }
    }
}
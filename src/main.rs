#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;

use crate::data_base::connection::{create_connection};

mod server;
mod gql_schema;
mod data_base;


#[tokio::main]
async fn main() {
    let pool = create_connection();
// Start the API server to handle requests.
    server::start(([127,0,0,1], 3030)).await;
}


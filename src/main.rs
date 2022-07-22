#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;

mod server;
mod gql_schema;
mod data_base;


#[tokio::main]
async fn main() {
    server::start(([127,0,0,1], 3030)).await;
}


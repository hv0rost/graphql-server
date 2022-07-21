#[macro_use]
extern crate diesel;
extern crate dotenv;

mod server;
mod gql_schema;
mod data_base;

extern crate log;

use crate::data_base::connection::establish_connection;

#[tokio::main]
async fn main() {
    //establish_connection();

// Start the API server to handle requests.
    server::start(([127,0,0,1], 3030)).await;
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod server;
mod gql_schema;
mod data_base;

extern crate log;

use crate::data_base::connection::establish_connection;
use diesel::QueryDsl;
use crate::data_base::models::*;
use self::diesel::prelude::*;

#[tokio::main]
async fn main() {
//init logger
    use data_base::schema::test::dsl::*;

    let connection = establish_connection();

    let results = test
        .limit(5)
        .load::<Test>(&connection)
        .expect("Error loading posts");
    println!("Displaying {} tests", results.len());

// Start the API server to handle requests.
    server::start(([127,0,0,1], 3030)).await;
}

use std::env;
use std::sync::Arc;
use async_graphql::Context;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("{}",database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> PgConnection {
    ctx.data::<Arc<PgConnection>>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}
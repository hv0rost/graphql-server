use std::env;
use std::sync::Arc;
use async_graphql::Context;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool connection")
}

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> Conn {
    ctx.data::<Arc<PgPool>>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}
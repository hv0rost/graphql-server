use diesel::prelude::*;
use async_graphql::{Context, Object};
use crate::data_base::{db_req, models::*, schema::test};
use crate::data_base::connection::get_conn_from_ctx;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery{
    async fn health(&self, ctx: &Context<'_>) -> Vec<Test>{
        db_req::get_all_tests(&get_conn_from_ctx(ctx))
    }
}
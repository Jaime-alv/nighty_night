use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::RunQueryDsl;

use crate::{
    model::baby_model::{Baby, InsertableBaby},
    schema::babies,
};

use super::connection_psql::establish_async_connection;

pub async fn ingest_new_baby_in_db<T>(new_baby: T) -> Result<Baby, Error>
where
    T: Into<InsertableBaby>,
{
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(babies::table)
        .values(new_baby.into())
        .returning(Baby::as_returning())
        .get_result(conn).await
    // .execute(conn)
}

pub async fn load_baby_by_id(id: i32) -> Result<Baby, Error> {
    let conn = &mut establish_async_connection().await;
    babies::table.find(id).first(conn).await
}

pub async fn query_babies() -> Result<Vec<Baby>, Error> {
    let conn = &mut establish_async_connection().await;
    babies::table.load(conn).await
}

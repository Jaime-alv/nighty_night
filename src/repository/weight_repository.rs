use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;

use crate::{
    model::weight_model::{InsertableWeight, Weight},
    schema::weights,
};

use super::connection_psql::establish_async_connection;

pub async fn ingest_weight<T>(new_measure: T) -> Result<usize, Error>
where
    T: Into<InsertableWeight>,
{
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(weights::table)
        .values(new_measure.into())
        .execute(conn).await
}

pub async fn get_all_weights_from_baby(baby: i32) -> Result<Vec<Weight>, Error> {
    let conn = &mut establish_async_connection().await;
    weights::table.filter(weights::baby_id.eq(baby)).load(conn).await
}

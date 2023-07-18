use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;

use crate::{
    model::weight_model::{InsertableWeight, Weight},
    schema::weights, data::weight_dto::UpdateWeight,
};

use super::connection_psql::establish_async_connection;

pub async fn ingest_weight<T>(new_measure: T) -> Result<usize, Error>
where
    T: Into<InsertableWeight>,
{
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(weights::table)
        .values(new_measure.into())
        .execute(conn)
        .await
}

pub async fn get_all_weights_from_baby(baby: i32) -> Result<Vec<Weight>, Error> {
    let conn = &mut establish_async_connection().await;
    weights::table
        .filter(weights::baby_id.eq(baby))
        .load(conn)
        .await
}

pub async fn patch_weight_record(record: i32, measure: UpdateWeight) -> Result<usize, Error> {
    let conn = &mut establish_async_connection().await;
    diesel::update(weights::table.find(record))
        .set((
            weights::date.eq(measure.date),
            weights::value.eq(measure.value),
        ))
        .execute(conn)
        .await
}

pub async fn find_weight_by_id(id: i32) -> Result<Weight, Error> {
    let conn = &mut establish_async_connection().await;
    weights::table.find(id).first(conn).await
}

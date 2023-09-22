use chrono::NaiveDate;
use diesel::{prelude::*, result::Error};

use crate::{
    data::{query_dto::Pagination, weight_dto::UpdateWeight},
    model::weight_model::{InsertableWeight, Weight},
    schema::weights,
};

use super::paginator::Paginate;
use crate::connection::connection_psql::establish_connection;

pub fn insert_new_weight<T>(new_measure: T) -> Result<usize, Error>
where
    T: Into<InsertableWeight>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(weights::table)
        .values(new_measure.into())
        .execute(conn)
}

pub fn select_all_weights_from_baby(
    baby: i32,
    pagination: Pagination,
) -> Result<(Vec<Weight>, i64), Error> {
    let conn = &mut establish_connection();
    weights::table
        .filter(weights::baby_id.eq(baby))
        .order(weights::date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn update_weight(record: i32, measure: UpdateWeight) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(weights::table.find(record))
        .set((
            weights::date.eq(measure.date),
            weights::value.eq(measure.value),
        ))
        .execute(conn)
}

pub fn select_weight_by_id(id: i32) -> Result<Weight, Error> {
    let conn = &mut establish_connection();
    weights::table.find(id).first(conn)
}

pub fn delete_weight(record: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(weights::table.find(record)).execute(conn)
}

pub fn select_weights_with_pagination(
    baby_id: i32,
    from: NaiveDate,
    to: NaiveDate,
    pagination: Pagination,
) -> Result<(Vec<Weight>, i64), Error> {
    let conn = &mut establish_connection();
    weights::table
        .filter(weights::baby_id.eq(baby_id))
        .filter(weights::date.ge(from))
        .filter(weights::date.le(to))
        .order(weights::date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

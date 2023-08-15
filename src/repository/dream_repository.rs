use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, result::Error};

use crate::{
    data::{dream_dto::UpdateDream, query_dto::Pagination},
    model::dream_model::{Dream, InsertableDream},
    schema::dreams,
    utils::datetime::now,
};

use super::{connection_psql::establish_connection, paginator::Paginate};

pub fn ingest_new_dream<T>(new_dream: T) -> Result<usize, Error>
where
    T: Into<InsertableDream>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(dreams::table)
        .values(new_dream.into())
        .execute(conn)
}

pub fn get_all_dreams_from_baby(
    baby: i32,
    pagination: Pagination,
) -> Result<(Vec<Dream>, u32), Error> {
    let conn = &mut establish_connection();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .order(dreams::from_date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

/// Filter table dreams by baby_id, where to_date is null and order
/// in descending to get the higher one.
pub fn get_last_dream(baby: i32) -> Result<Dream, Error> {
    let conn = &mut establish_connection();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::to_date.is_null())
        .order(dreams::from_date.desc())
        .first(conn)
}

pub fn update_last_dream(dream: InsertableDream) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    let last_dream = get_last_dream(dream.baby_id());
    diesel::update(dreams::table.filter(dreams::id.eq(last_dream.unwrap().id())))
        .set(dreams::to_date.eq(dream.to_date()))
        .execute(conn)
}

/// Only need dates that have both fields, from_date and to_date, because we need to sum durations.
pub fn find_dreams_summary(baby: i32, from: NaiveDate, to: NaiveDate) -> Result<Vec<Dream>, Error> {
    let conn = &mut establish_connection();
    let from_timestamp = from.and_hms_opt(0, 0, 1).unwrap();
    let to_timestamp = to.and_hms_opt(23, 59, 59).unwrap();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::to_date.le(to_timestamp))
        .filter(dreams::to_date.ge(from_timestamp))
        .load::<Dream>(conn)
}

pub fn find_dream_by_id(id: i32) -> Result<Dream, Error> {
    let conn = &mut establish_connection();
    dreams::table.find(id).first(conn)
}

pub fn patch_dream_record(record_id: i32, dream: UpdateDream) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(dreams::table.find(record_id))
        .set((
            dreams::from_date.eq(dream.from_date),
            dreams::to_date.eq(dream.to_date),
        ))
        .execute(conn)
}

pub fn delete_dream_from_db(record_id: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(dreams::table.find(record_id)).execute(conn)
}

pub fn dreams_paginated_from_db(
    baby_id: i32,
    pagination: Pagination,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<(Vec<Dream>, u32), Error> {
    let conn = &mut establish_connection();
    let from_timestamp = from.and_hms_opt(0, 0, 1).unwrap();
    let to_timestamp = to.and_hms_opt(23, 59, 59).unwrap();
    dreams::table
        .filter(dreams::baby_id.eq(baby_id))
        .filter(dreams::to_date.le(to_timestamp))
        .filter(dreams::to_date.ge(from_timestamp))
        .order(dreams::from_date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

/*
Get first Option<date> and last Option<date> for a baby id, if both records are null, default
to today date.
 */
pub fn obtain_first_and_last_dream_date(baby: i32) -> Result<(NaiveDate, NaiveDate), Error> {
    let conn = &mut establish_connection();
    let start: Option<NaiveDateTime> = dreams::table
        .filter(dreams::baby_id.eq(baby))
        .select(dreams::to_date)
        .filter(dreams::to_date.is_not_null())
        .order(dreams::to_date.asc())
        .first(conn)?;
    let stop: Option<NaiveDateTime> = dreams::table
        .filter(dreams::baby_id.eq(baby))
        .select(dreams::to_date)
        .filter(dreams::to_date.is_not_null())
        .order(dreams::to_date.desc())
        .first(conn)?;
    Ok((start.unwrap_or(now()).date(), stop.unwrap_or(now()).date()))
}

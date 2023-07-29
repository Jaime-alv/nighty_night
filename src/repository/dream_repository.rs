use chrono::{Days, NaiveDate, NaiveDateTime};
use diesel::{prelude::*, result::Error};

use crate::{
    data::{dream_dto::UpdateDream, query_dto::Pagination},
    model::dream_model::{Dream, InsertableDream},
    schema::dreams,
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

pub fn find_dreams_by_date(
    baby: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<Dream>, Error> {
    let conn = &mut establish_connection();
    let down_time = from_date.and_hms_opt(0, 0, 1).unwrap();
    let up_time = to_date.and_hms_opt(23, 59, 59).unwrap();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::from_date.ge(down_time))
        .filter(dreams::from_date.le(up_time))
        .order(dreams::from_date.asc())
        .load::<Dream>(conn)
}

fn find_last_dream_from_yesterday(baby: i32, date: NaiveDate) -> NaiveDateTime {
    let conn = &mut establish_connection();
    let yesterday = date.checked_sub_days(Days::new(1)).unwrap();
    let min_date = yesterday.and_hms_opt(12, 0, 1).unwrap();
    let max_date = yesterday.and_hms_opt(23, 59, 29).unwrap();
    let dream: Result<Dream, Error> = dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::from_date.gt(min_date))
        .filter(dreams::from_date.lt(max_date))
        .order(dreams::from_date.desc())
        .first::<Dream>(conn);
    match dream {
        Ok(dream) => dream.from_date(),
        Err(_) => date.and_hms_opt(0, 0, 1).unwrap(),
    }
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

pub fn find_first_record(baby: i32) -> Result<NaiveDate, Error> {
    let conn = &mut establish_connection();
    let first_record: Dream = dreams::table
        .filter(dreams::baby_id.eq(baby))
        .order(dreams::from_date.asc())
        .first::<Dream>(conn)?;
    Ok(first_record.to_date().unwrap().date())
}

pub fn find_all_dreams_sorted(baby: i32) -> Result<Vec<Dream>, Error> {
    let conn = &mut establish_connection();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::to_date.is_not_null())
        .order(dreams::to_date.asc())
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

pub fn count_dreams() -> Result<i64, Error> {
    let conn = &mut establish_connection();
    dreams::table.select(dreams::id).count().get_result(conn)
}

use chrono::{Days, NaiveDate, NaiveDateTime};
use diesel::{prelude::*, result::Error};

use crate::{
    model::dream_model::{Dream, InsertableDream},
    schema::dreams,
};

use super::connection_psql::establish_connection;

pub fn ingest_new_dream<T>(new_dream: T) -> Result<usize, Error>
where
    T: Into<InsertableDream>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(dreams::table)
        .values(new_dream.into())
        .execute(conn)
}

pub fn get_all_dreams_from_baby(baby: i32) -> Result<Vec<Dream>, Error> {
    let conn = &mut establish_connection();
    dreams::table.filter(dreams::baby_id.eq(baby)).load(conn)
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

pub fn find_dreams_by_date(baby: i32, date: NaiveDate) -> Result<Vec<Dream>, Error> {
    let timestamp = date.and_hms_opt(23, 59, 59).unwrap();
    let last_dream_from_yesterday = find_last_dream_from_yesterday(baby, date);
    let conn = &mut establish_connection();
    dreams::table
        .filter(dreams::baby_id.eq(baby))
        .filter(dreams::to_date.lt(timestamp))
        .filter(dreams::from_date.ge(last_dream_from_yesterday))
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

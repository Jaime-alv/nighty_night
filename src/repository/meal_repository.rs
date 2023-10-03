use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, result::Error};
// use diesel_::RunQueryDsl;

use crate::{
    data::query_dto::Pagination,
    model::meals_model::{InsertableMeal, Meal},
    schema::meals,
};

use super::paginator::Paginate;
use crate::connection::connection_psql::establish_connection;

pub fn insert_new_meal<T>(new_meal: T) -> Result<Meal, Error>
where
    T: Into<InsertableMeal>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(meals::table)
        .values(new_meal.into())
        .get_result(conn)
}

pub fn select_all_meals_from_baby(
    baby: i32,
    pagination: Pagination,
) -> Result<(Vec<Meal>, i64), Error> {
    let conn = &mut establish_connection();
    meals::table
        .filter(meals::baby_id.eq(baby))
        .order(meals::date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn select_meals_by_date_range(
    baby: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_connection();
    let from = from_date.and_hms_opt(0, 0, 1).unwrap();
    let to = to_date.and_hms_opt(23, 59, 59).unwrap();
    meals::table
        .filter(meals::baby_id.eq(baby))
        .filter(meals::date.ge(from))
        .filter(meals::date.le(to))
        .order(meals::date.asc())
        .load::<Meal>(conn)
}

pub fn select_meal_by_id(record: i32) -> Result<Meal, Error> {
    let conn = &mut establish_connection();
    meals::table.find(record).first::<Meal>(conn)
}

pub fn update_meal(meal: Meal) -> Result<Meal, Error> {
    let conn = &mut establish_connection();
    diesel::update(meals::table.find(meal.id()))
        .set((
            meals::date.eq(meal.date()),
            meals::quantity.eq(meal.quantity()),
            meals::to_time.eq(meal.to_time()),
        ))
        .get_result(conn)
}

pub fn delete_meal(record: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(meals::table.find(record)).execute(conn)
}

pub fn select_meals_with_pagination(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<(Vec<Meal>, i64), Error> {
    let conn = &mut establish_connection();
    let from = from_date.and_hms_opt(0, 0, 1).unwrap();
    let to = to_date.and_hms_opt(23, 59, 59).unwrap();
    meals::table
        .filter(meals::baby_id.eq(baby_id))
        .filter(meals::date.ge(from))
        .filter(meals::date.le(to))
        .order(meals::date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn select_date_first_and_last_meal(baby: i32) -> Result<(NaiveDate, NaiveDate), Error> {
    let conn = &mut establish_connection();
    let start: NaiveDateTime = meals::table
        .filter(meals::baby_id.eq(baby))
        .select(meals::date)
        .order(meals::date.asc())
        .first(conn)?;
    let stop: NaiveDateTime = meals::table
        .filter(meals::baby_id.eq(baby))
        .select(meals::date)
        .order(meals::date.desc())
        .first(conn)?;
    Ok((start.date(), stop.date()))
}

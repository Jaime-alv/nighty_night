use chrono::NaiveDate;
use diesel::{prelude::*, result::Error};
// use diesel_::RunQueryDsl;

use crate::{
    data::{meal_dto::UpdateMeal, query_dto::Pagination},
    model::meals_model::{InsertableMeal, Meal},
    schema::meals,
};

use super::{
    connection_psql::establish_connection,
    paginator::{Paginate, Paginated},
};

pub fn ingest_meal<T>(new_meal: T) -> Result<usize, Error>
where
    T: Into<InsertableMeal>,
{
    let conn = &mut establish_connection();
    diesel::insert_into(meals::table)
        .values(new_meal.into())
        .execute(conn)
}

pub fn get_all_meals_from_baby(
    baby: i32,
    pagination: Pagination,
) -> Result<(Vec<Meal>, u32), Error> {
    let conn = &mut establish_connection();
    meals::table
        .filter(meals::baby_id.eq(baby))
        .order(meals::date.asc())
        .paginate(pagination.page())
        .per_page(pagination.per_page())
        .load_and_count_pages(conn)
}

pub fn find_meals_by_date(baby: i32, date: NaiveDate) -> Result<Vec<Meal>, Error> {
    find_meals_by_date_range(baby, date, date)
}

pub fn find_meals_by_date_range(
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

pub fn find_all_meals_sorted(baby: i32) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_connection();
    meals::table
        .filter(meals::baby_id.eq(baby))
        .order(meals::date.asc())
        .load::<Meal>(conn)
}

pub fn find_meal_by_id(record: i32) -> Result<Meal, Error> {
    let conn = &mut establish_connection();
    meals::table.find(record).first::<Meal>(conn)
}

pub fn patch_meal_record(record: i32, meal: UpdateMeal) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::update(meals::table.find(record))
        .set((
            meals::date.eq(meal.date),
            meals::quantity.eq(meal.quantity),
            meals::to_time.eq(meal.to_time),
        ))
        .execute(conn)
}

pub fn delete_meal_from_db(record: i32) -> Result<usize, Error> {
    let conn = &mut establish_connection();
    diesel::delete(meals::table.find(record)).execute(conn)
}

pub fn meals_paginated_from_db(
    baby_id: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
    pagination: Pagination,
) -> Result<(Vec<Meal>, u32), Error> {
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

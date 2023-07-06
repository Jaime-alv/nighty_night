use chrono::NaiveDate;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;

use crate::{
    model::meals_model::{InsertableMeal, Meal},
    schema::meals,
};

use super::connection_psql::establish_async_connection;

pub async fn ingest_meal<T>(new_meal: T) -> Result<usize, Error>
where
    T: Into<InsertableMeal>,
{
    let conn = &mut establish_async_connection().await;
    diesel::insert_into(meals::table)
        .values(new_meal.into())
        .execute(conn)
        .await
}

pub async fn get_all_meals_from_baby(baby: i32) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_async_connection().await;
    meals::table
        .filter(meals::baby_id.eq(baby))
        .load(conn)
        .await
}

pub async fn find_meals_by_date(baby: i32, date: NaiveDate) -> Result<Vec<Meal>, Error> {
    find_meals_by_date_range(baby, date, date).await
}

pub async fn find_meals_by_date_range(
    baby: i32,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_async_connection().await;
    let from = from_date.and_hms_opt(0, 0, 1).unwrap();
    let to = to_date.and_hms_opt(23, 59, 59).unwrap();
    meals::table
        .filter(meals::baby_id.eq(baby))
        .filter(meals::date.ge(from))
        .filter(meals::date.le(to))
        .load::<Meal>(conn)
        .await
}

pub async fn find_all_meals_sorted(baby: i32) -> Result<Vec<Meal>, Error> {
    let conn = &mut establish_async_connection().await;
    meals::table
        .filter(meals::baby_id.eq(baby))
        .order(meals::date.asc())
        .load::<Meal>(conn)
        .await
}

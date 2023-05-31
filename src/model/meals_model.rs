use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable, Insertable};

use crate::schema::meals;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = meals)]
pub struct Meal {
    id: i32,
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    elapsed: Option<i16>
    
}

#[derive(Insertable)]
#[diesel(table_name= meals)]
pub struct InsertableMeal {
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    elapsed: Option<i16>
}
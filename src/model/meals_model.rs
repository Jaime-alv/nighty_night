use chrono::{Duration, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable};

use crate::{
    schema::meals,
    utils::datetime::{format_date, format_duration, format_time},
};

#[derive(Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = meals)]
pub struct Meal {
    id: i32,
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    to_time: Option<NaiveDateTime>,
}

impl Meal {
    pub fn new(
        id: i32,
        baby_id: i32,
        date: NaiveDateTime,
        quantity: Option<i16>,
        to_time: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            baby_id,
            date,
            quantity,
            to_time,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn baby_id(&self) -> i32 {
        self.baby_id
    }

    pub fn date(&self) -> NaiveDateTime {
        self.date
    }

    pub fn quantity(&self) -> Option<i16> {
        self.quantity
    }

    pub fn to_time(&self) -> Option<NaiveDateTime> {
        self.to_time
    }
    
    pub fn formatted_quantity(&self) -> i16 {
        match self.quantity {
            Some(q) => q,
            None => 0,
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.to_time {
            Some(e) => e - self.date,
            None => Duration::seconds(0),
        }
    }

    pub fn formatted_elapsed(&self) -> String {
        let duration: i64 = Self::elapsed(&self).num_minutes();
        format_duration(duration)
    }

    pub fn formatted_date(&self) -> String {
        format_date(self.date.date())
    }

    pub fn formatted_time(&self) -> String {
        format_time(self.date.time())
    }
}

#[derive(Insertable)]
#[diesel(table_name= meals)]
pub struct InsertableMeal {
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    to_time: Option<NaiveDateTime>,
}

impl InsertableMeal {
    pub fn new(
        baby_id: i32,
        date: NaiveDateTime,
        quantity: Option<i16>,
        to_time: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            baby_id,
            date,
            quantity,
            to_time,
        }
    }
}

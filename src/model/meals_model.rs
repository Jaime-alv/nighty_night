use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};

use crate::{
    schema::meals,
    utils::datetime::{format_date, format_time},
};

#[derive(Queryable, Identifiable)]
#[diesel(table_name = meals)]
pub struct Meal {
    id: i32,
    baby_id: i32,
    date: NaiveDateTime,
    quantity: Option<i16>,
    elapsed: Option<i16>,
}

impl Meal {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn date(&self) -> NaiveDateTime {
        self.date
    }

    pub fn quantity(&self) -> i16 {
        match self.quantity {
            Some(q) => q,
            None => 0,
        }
    }

    pub fn elapsed(&self) -> i16 {
        match self.elapsed {
            Some(e) => e,
            None => 0,
        }
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
    elapsed: Option<i16>,
}

impl InsertableMeal {
    pub fn new(
        baby_id: i32,
        date: NaiveDateTime,
        quantity: Option<i16>,
        elapsed: Option<i16>,
    ) -> Self {
        Self {
            baby_id,
            date,
            quantity,
            elapsed,
        }
    }
}

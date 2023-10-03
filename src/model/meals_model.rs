use chrono::{Duration, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable};

use crate::{
    data::meal_dto::InputMealDto,
    schema::meals,
    utils::datetime::{
        convert_to_date_time, format_date, format_duration, format_time,
        parse_string_to_optional_date,
    },
};

#[derive(Queryable, Identifiable, Clone)]
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
        let time = match self.to_time {
            Some(value) => value - self.date,
            None => Duration::minutes(0),
        };
        format_duration(time.num_minutes())
    }

    pub fn formatted_date(&self) -> String {
        format_date(self.date.date())
    }

    pub fn formatted_time(&self) -> String {
        format_time(self.date.time())
    }

    /// Update with new fields.
    ///
    /// If new date gives a parsing error, defaults to older value.
    pub fn update_meal(&self, new_meal: InputMealDto) -> Self {
        let new_date = match new_meal.date {
            Some(v) => convert_to_date_time(&v).unwrap_or(self.date),
            None => self.date,
        };
        let new_quantity = match new_meal.quantity {
            Some(value) => {
                if value.eq(&0) {
                    None
                } else {
                    Some(value)
                }
            }
            None => self.quantity,
        };

        /*
         * Cast from Option<String> to Option<NaiveDateTime>.
         * If there are any errors found, it will default to None
         */
        let new_to_time: Option<NaiveDateTime> = match new_meal.to_time {
            Some(to_time_value) => parse_string_to_optional_date(new_date, &to_time_value),
            None => self.to_time,
        };
        Self {
            date: new_date,
            quantity: new_quantity,
            to_time: new_to_time,
            ..self.clone()
        }
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

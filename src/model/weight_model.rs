use chrono::NaiveDate;
use diesel::{Identifiable, Insertable, Queryable};

use crate::{
    data::weight_dto::InputWeightDto,
    schema::weights,
    utils::datetime::{convert_to_date, format_date},
};

#[derive(Queryable, Identifiable, Clone)]
#[diesel(table_name = weights)]
pub struct Weight {
    id: i32,
    baby_id: i32,
    date: NaiveDate,
    value: f32,
}

impl Weight {
    pub fn new(id: i32, baby_id: i32, date: NaiveDate, value: f32) -> Self {
        Self {
            id,
            baby_id,
            date,
            value,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn baby_id(&self) -> i32 {
        self.baby_id
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn formatted_date(&self) -> String {
        format_date(self.date)
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn update_weight(&self, new_weight: InputWeightDto) -> Self {
        let new_date = match new_weight.date {
            Some(value) => convert_to_date(&value).unwrap_or(self.date),
            None => self.date,
        };
        let new_measure = match new_weight.value {
            Some(value) => value,
            None => self.value,
        };
        Self {
            date: new_date,
            value: new_measure,
            ..self.clone()
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = weights)]
pub struct InsertableWeight {
    baby_id: i32,
    date: NaiveDate,
    value: f32,
}

impl InsertableWeight {
    pub fn new(baby_id: i32, date: NaiveDate, value: f32) -> Self {
        Self {
            baby_id,
            date,
            value,
        }
    }
}

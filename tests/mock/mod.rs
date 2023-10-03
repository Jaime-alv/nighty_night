pub mod entities;

use std::ops::Range;

use chrono::{DateTime, TimeZone, Utc};
use fake::{
    faker::{
        chrono::en::DateTimeBetween,
        internet::en::{FreeEmail, Password, Username},
        name::en::{FirstName, LastName},
    },
    Fake,
};
use nighty_night::data::{baby_dto::InputBabyDto, user_dto::NewUserDto};

/// Generate random user with all required and
/// optional fields.
pub fn generate_new_user() -> NewUserDto {
    NewUserDto {
        username: Username().fake(),
        password: Password(Range { start: 8, end: 10 }).fake(),
        email: FreeEmail().fake(),
        name: FirstName().fake(),
        surname: LastName().fake(),
    }
}

pub fn generate_date() -> String {
    let today = Utc::now();
    let minimum_date = Utc.with_ymd_and_hms(2018, 1, 1, 00, 00, 00).unwrap();
    let date: String = DateTimeBetween(minimum_date, today).fake();
    let timestamp = DateTime::parse_from_rfc3339(&date).unwrap().date_naive();
    timestamp.to_string()
}

pub fn generate_new_baby() -> InputBabyDto {
    let birthdate = generate_date();
    InputBabyDto {
        name: Some(FirstName().fake()),
        birthdate: Some(birthdate),
    }
}

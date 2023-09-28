use std::ops::Range;

use fake::{
    faker::{
        internet::en::{FreeEmail, Password, Username},
        name::en::{FirstName, LastName},
    },
    Fake,
};
use nighty_night::data::user_dto::{NewUserDto, UpdateUserDto};

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

/// Generate update user fields.
pub fn generate_update_user() -> UpdateUserDto {
    UpdateUserDto {
        email: FreeEmail().fake(),
        name: FirstName().fake(),
        surname: LastName().fake(),
    }
}

pub fn generate_empty_user() -> NewUserDto {
    NewUserDto {
        username: "".to_string(),
        password: "abc".to_string(),
        email: None,
        name: None,
        surname: None,
    }
}

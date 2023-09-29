use std::ops::Range;

use fake::{
    faker::{
        internet::en::{FreeEmail, Password, Username},
        name::en::{FirstName, LastName},
    },
    Fake,
};
use nighty_night::data::user_dto::{LoginDto, NewUserDto, UpdateUserDto};

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
    let email: String = FreeEmail().fake();
    let name: String = FirstName().fake();
    let surname: String = LastName().fake();
    UpdateUserDto {
        email: Some(email),
        name: Some(name),
        surname: Some(surname),
    }
}

pub fn generate_login_credentials(username: &str, password: &str) -> LoginDto {
    LoginDto {
        username: username.to_string(),
        password: password.to_string(),
    }
}

pub fn generate_invalid_credentials(username: Option<&str>, password: Option<&str>) -> LoginDto {
    let username_field: String = if username.is_none() {
        Username().fake()
    } else {
        username.unwrap().to_string()
    };
    let password_field: String = if password.is_none() {
        Password(Range { start: 6, end: 7 }).fake()
    } else {
        password.unwrap().to_string()
    };
    LoginDto {
        username: username_field,
        password: password_field,
    }
}
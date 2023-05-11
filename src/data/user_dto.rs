use serde::{Deserialize, Serialize};

use super::traits::Mandatory;

#[derive(Deserialize)]
pub struct NewUserDto {
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: Option<String>,
    pub surname: Option<String>,
}

impl Mandatory for NewUserDto {
    fn data(&self) -> Vec<&str> {
        vec![
            self.username.as_str(),
            self.password.as_str(),
            self.email.as_str(),
        ]
    }
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    username: String,
    email: String,
    name: Option<String>,
    surname: Option<String>,
    babies: Vec<String>,
}

impl UserDto {
    pub fn new(
        username: String,
        email: String,
        name: Option<String>,
        surname: Option<String>,
        babies: Vec<String>,
    ) -> Self {
        Self {
            username,
            email,
            name,
            surname,
            babies,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FindUserDto {
    pub username: String,
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

impl Mandatory for LoginDto {
    fn data(&self) -> Vec<&str> {
        vec![self.username.as_str(), self.password.as_str()]
    }
}
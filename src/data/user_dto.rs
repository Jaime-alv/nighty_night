use serde::{Deserialize, Serialize};

use super::traits::Mandatory;

#[derive(Deserialize)]
pub struct NewUserDto {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
}

impl Mandatory for NewUserDto {
    fn data(&self) -> Vec<&str> {
        vec![self.username.as_str(), self.password.as_str()]
    }
}

#[derive(Serialize)]
pub struct UserData {
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
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

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
}

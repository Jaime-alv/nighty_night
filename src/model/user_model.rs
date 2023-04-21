use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::user_model;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = user_model)]
pub struct User {
    pub user_id: i32,
    username: String,
    password: String,
    rol: i32,
    task: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = user_model)]
pub struct NewUserDto {
    username: String,
    password: String,
    rol: Option<i32>,
}

impl User {
    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_id(&self) -> i32 {
        self.user_id
    }

    pub fn query_users(conn: &mut SqliteConnection) -> Vec<Self> {
        user_model::table.load(conn).unwrap()
    }

    pub fn load_user(
        conn: &mut SqliteConnection,
        user: String,
    ) -> Result<Self, diesel::result::Error> {
        user_model::table
            .filter(user_model::username.eq(user))
            .first(conn)
    }

    pub fn load_user_by_id(
        conn: &mut SqliteConnection,
        user_id: i32,
    ) -> Result<Self, diesel::result::Error> {
        user_model::table.find(user_id).first(conn)
    }

    pub fn user_exists(conn: &mut SqliteConnection, user: String) -> bool {
        let user = Self::load_user(conn, user);
        match user {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
}

impl NewUserDto {
    pub fn new(username: String, password: String, rol: Option<i32>) -> Self {
        Self {
            username,
            password,
            rol,
        }
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }

    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn get_rol(&self) -> Option<i32> {
        self.rol
    }
    pub fn create_user(&self, conn: &mut SqliteConnection) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(user_model::table)
            .values(self)
            .execute(conn)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub username: String,
}

impl UserDto {
    pub fn new(user: &User) -> Self {
        Self {
            username: user.get_username(),
        }
    }

    pub fn user_to_user_dto_array(users: Vec<User>) -> Vec<Self> {
        users.into_iter().map(|u| Self::new(&u)).collect()
    }
}

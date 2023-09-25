use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    data::user_dto::UpdateUserDto, repository::user_repository::select_roles_id_from_user,
    schema::users, security::security::verify_password, utils::datetime::now,
};

#[derive(Queryable, Selectable, Identifiable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    name: Option<String>,
    surname: Option<String>,
    email: Option<String>,
    active: bool,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(
        id: i32,
        username: String,
        password: String,
        name: Option<String>,
        surname: Option<String>,
        email: Option<String>,
        active: bool,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            username,
            password,
            name,
            surname,
            email,
            active,
            created_at,
            updated_at,
        }
    }

    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> Option<String> {
        self.email.to_owned()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }

    pub fn is_password_match(&self, input_password: &str) -> bool {
        verify_password(self.password.to_owned(), input_password)
    }

    pub fn name(&self) -> Option<String> {
        self.name.to_owned()
    }

    pub fn surname(&self) -> Option<String> {
        self.surname.to_owned()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn roles(&self) -> Vec<String> {
        let roles = select_roles_id_from_user(self.id).unwrap();
        roles
            .into_iter()
            .map(|rol| match rol {
                0 => "admin".to_string(),
                1 => "user".to_string(),
                2 => "anonymous".to_string(),
                _ => "undefined".to_string(),
            })
            .collect()
    }

    pub fn update_profile(&self, profile: UpdateUserDto) -> Self {
        let new_name = match profile.name {
            Some(value) => Some(value),
            None => self.name(),
        };
        let new_surname = match profile.surname {
            Some(value) => Some(value),
            None => self.surname(),
        };
        let new_email = match profile.email {
            Some(value) => Some(value),
            None => self.email(),
        };
        let update_time = Some(now());
        Self {
            name: new_name,
            surname: new_surname,
            email: new_email,
            updated_at: update_time,
            ..self.clone()
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    username: String,
    password: String,
    email: Option<String>,
    name: Option<String>,
    surname: Option<String>,
    active: bool,
    created_at: NaiveDateTime,
}

impl InsertableUser {
    pub fn new(
        username: String,
        password: String,
        email: Option<String>,
        name: Option<String>,
        surname: Option<String>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            username,
            password,
            email,
            name,
            surname,
            active: true,
            created_at,
        }
    }
}

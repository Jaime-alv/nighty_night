use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    repository::user_repository::find_related_babies, schema::users,
    security::security::verify_password,
};

use super::baby_model::Baby;

#[derive(Queryable, Selectable, Identifiable)]
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
    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> Option<String> {
        self.email.to_owned()
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

    pub fn find_related_babies(&self) -> Vec<Baby> {
        find_related_babies(self)
    }

    pub fn find_related_babies_names(&self) -> Vec<String> {
        let babies = Self::find_related_babies(self);
        babies.iter().map(|baby: &Baby| baby.name()).collect()
    }

    pub fn active(&self) -> bool {
        self.active
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

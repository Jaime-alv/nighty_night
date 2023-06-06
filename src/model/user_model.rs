use std::collections::HashSet;

use diesel::prelude::*;

use crate::{
    repository::user_repository::{find_babies_id, find_related_babies, find_roles_id},
    schema::users,
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
    email: String,
    active: bool,
}

impl User {
    pub fn username(&self) -> String {
        self.username.to_string()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> String {
        self.email.to_string()
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

    pub fn find_roles_id(&self) -> HashSet<u8> {
        find_roles_id(self.id)
    }

    pub fn find_babies_id(&self) -> Vec<i32> {
        find_babies_id(self.id)
    }

    pub fn find_related_babies_names(&self) -> Vec<String> {
        let babies = Self::find_related_babies(self);
        babies.iter().map(|baby: &Baby| baby.name()).collect()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn has_baby(&self, baby_id: i32) -> bool {
        let babies = Self::find_babies_id(&self);
        babies.contains(&baby_id)
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    username: String,
    password: String,
    email: String,
    name: Option<String>,
    surname: Option<String>,
    active: bool,
}

impl InsertableUser {
    pub fn new(
        username: String,
        password: String,
        email: String,
        name: Option<String>,
        surname: Option<String>,
    ) -> Self {
        Self {
            username,
            password,
            email,
            name,
            surname,
            active: true,
        }
    }
}

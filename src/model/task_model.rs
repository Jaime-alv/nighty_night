use diesel::prelude::*;
use diesel::Queryable;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::model::user_model::User;

use crate::schema::task;

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Associations)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = task)]
pub struct Task {
    task_id: i32,
    user_id: i32,
    name: String,
    description: String,
    done: bool,
}

impl Task {
    
    pub fn last_inserted_row(conn: &mut SqliteConnection) -> Result<Self, diesel::result::Error> {
        task::table.order(task::task_id.desc()).first(conn)
    }

    pub fn get_all_tasks(conn: &mut SqliteConnection) -> Vec<Self> {
        task::table.load(conn).unwrap()
    }

    pub fn get_all_tasks_by_user(conn: &mut SqliteConnection, user_id: i32) -> Vec<Self> {
        task::table.filter(task::user_id.eq(user_id)).load(conn).unwrap()
    }

    pub fn get_task_by_id(conn: &mut SqliteConnection, user_id: i32, task_id: i32) -> Result<Self, diesel::result::Error> {
        task::table.filter(task::user_id.eq(user_id)).find(task_id).first(conn)
    }
}

#[derive(Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name = task)]
pub struct NewTaskDTO {
    pub name: String,
    pub description: String,
    done: Option<bool>,
    user_id: Option<i32>,
}

impl NewTaskDTO {
    pub fn new(
        name: String,
        description: String,
        done: Option<bool>,
        user_id: Option<i32>,
    ) -> Self {
        Self {
            name,
            description,
            done,
            user_id,
        }
    }

    pub fn insert_task(&self, conn: &mut SqliteConnection)-> Result<Task, StatusCode> {
    diesel::insert_into(task::table)
        .values(self)
        .execute(conn)
        .expect("Error while inserting task!");
    let last_task = Task::last_inserted_row(conn);
    match last_task {
        Ok(t) => Ok(t),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
}


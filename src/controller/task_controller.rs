use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Json,
};
use hyper::StatusCode;

use crate::{
    establish_connection,
    model::{
        task_model::{NewTaskDTO, Task},
        user_model::User,
    },
};

#[axum_macros::debug_handler]
pub async fn new_task(
    Path(user_id): Path<i32>,
    Json(tmp_task): Json<NewTaskDTO>,
) -> Result<Json<Task>, StatusCode> {
    let conn = &mut establish_connection();
    if tmp_task.description.is_empty() || tmp_task.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    };
    let data = NewTaskDTO::new(
        tmp_task.name,
        tmp_task.description,
        Some(false),
        Some(user_id),
    );
    let current_user = User::load_user_by_id(conn, user_id);
    let task = match current_user {
        Ok(_) => data.insert_task(conn),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    match task {
        Ok(t) => return Ok(Json(t)),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_task(Path(user_id): Path<i32>) -> Json<Vec<Task>> {
    let conn = &mut establish_connection();
    Json(Task::get_all_tasks_by_user(conn, user_id))
}

pub async fn get_all_tasks() -> Json<Vec<Task>> {
    let conn = &mut establish_connection();
    Json(Task::get_all_tasks(conn))
}

pub async fn get_task_by_id(
    Path(user_id): Path<i32>,
    Query(task_id): Query<HashMap<String, i32>>,
) -> Result<Json<Task>, StatusCode> {
    let conn = &mut establish_connection();
    match Task::get_task_by_id(conn, user_id, *task_id.get("task").unwrap()) {
        Ok(t) => Ok(Json(t)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

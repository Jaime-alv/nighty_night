use crate::establish_connection;
use axum::Json;
use hyper::StatusCode;

use crate::model::user_model::{NewUserDto, User, UserDto};

pub async fn register_new_user(Json(new_user): Json<NewUserDto>) -> Result<Json<User>, StatusCode> {
    let conn = &mut establish_connection();
    if new_user.get_username().is_empty() || new_user.get_password().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if User::user_exists(conn, new_user.get_username()) {
        return Err(StatusCode::FOUND);
    }
    let new_user = match new_user.get_rol() {
        Some(_) => new_user,
        None => NewUserDto::new(
            new_user.get_username(),
            new_user.get_password(),
            Some(1),
        ),
    };

    match new_user.create_user(conn) {
        Ok(u) => return Ok(Json(u)),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_all_users() -> Result<Json<Vec<UserDto>>, StatusCode> {
    let users = User::query_users(&mut establish_connection());
    if users.len() > 0 {
        return Ok(Json(UserDto::user_to_user_dto_array(users)));
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn find_user(Json(data): Json<UserDto>) -> Result<Json<User>, StatusCode> {
    let tmp_user = User::load_user(&mut establish_connection(), data.username);
    match tmp_user {
        Ok(v) => return Ok(Json(v)),
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    }
}

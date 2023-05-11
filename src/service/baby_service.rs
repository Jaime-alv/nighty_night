// use diesel::{result::Error, SqliteConnection};
// use diesel::prelude::*;

// use crate::{data::baby_dto::NewBabyDto, model::baby_model::Baby, schema::babies};

// pub fn get_all_babies_by_user(user_id: i32) -> Vec<BabyUser> {
//     let conn = &mut establish_connection();
//     babies::table
//         .filter(babies::user_id.eq(user_id))
//         .load(conn)
//         .unwrap()
// }

use crate::{
    data::baby_dto::{BabyDto, NewBabyDto},
    error::error::ApiError,
    model::user_model::User,
    repository::baby_repository::{ingest_new_baby_in_db, load_baby_by_id, query_babies}, mapping::baby_mapper::babies_to_babies_dto,
};

pub async fn ingest_new_baby(
    new_baby: NewBabyDto,
    current_user: User,
) -> Result<BabyDto, ApiError> {
    match ingest_new_baby_in_db(new_baby) {
        Ok(baby) => Ok(BabyDto::from(baby)),
        Err(_) => Err(ApiError::DBError),
    }
}

pub async fn find_baby_service(baby_id: i32) -> Result<BabyDto, ApiError> {
    match load_baby_by_id(baby_id) {
        Ok(baby) => Ok(BabyDto::from(baby)),
        Err(_) => Err(ApiError::NoEntryFound),
    }
}

pub async fn get_all_babies_service() -> Result<Vec<BabyDto>, ApiError> {
    match query_babies() {
        Ok(babies) => Ok(babies_to_babies_dto(babies)),
        Err(_) => Err(ApiError::DBError),
    }
}

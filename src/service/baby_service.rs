use tracing::error;

use crate::{
    data::baby_dto::{BabyDto, NewBabyDto},
    error::error::ApiError,
    mapping::baby_mapper::babies_to_babies_dto,
    repository::baby_repository::{ingest_new_baby_in_db, load_baby_by_id, query_babies},
};

use super::association_service::add_baby_to_user_service;

pub async fn ingest_new_baby<T>(new_baby: NewBabyDto, current_user: T) -> Result<BabyDto, ApiError>
where
    T: Into<i32>,
{
    match ingest_new_baby_in_db(new_baby).await {
        Ok(baby) => {
            match add_baby_to_user_service(current_user.into(), baby.id().into()).await {
                Ok(_) => (),
                Err(msg) => return Err(msg),
            };
            Ok(BabyDto::from(baby))
        }
        Err(msg) => {
            error!("{msg}");
            Err(ApiError::DBError(msg))
        }
    }
}

pub async fn find_baby_service(baby_id: i32) -> Result<BabyDto, ApiError> {
    match load_baby_by_id(baby_id).await {
        Ok(baby) => Ok(BabyDto::from(baby)),
        Err(_) => Err(ApiError::NoEntryFound),
    }
}

pub async fn get_all_babies_service() -> Result<Vec<BabyDto>, ApiError> {
    match query_babies().await {
        Ok(babies) => Ok(babies_to_babies_dto(babies).await),
        Err(msg) => {
            error!("{msg}");
            Err(ApiError::DBError(msg))
        }
    }
}

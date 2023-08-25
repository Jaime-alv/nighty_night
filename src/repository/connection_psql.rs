use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::configuration::settings::Setting;
use crate::response::error::ApiError;

pub fn establish_connection() -> PgConnection {
    let database_url = Setting::DatabaseUrl.get();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn check_db_status() -> Result<String, ApiError> {
    let database_url = Setting::DatabaseUrl.get();
    match PgConnection::establish(&database_url) {
        Ok(_) => Ok("PostgreSQL ready.".to_string()),
        Err(error) => Err(ApiError::Generic500Error(format!("PostgreSQL: {error}"))),
    }
}

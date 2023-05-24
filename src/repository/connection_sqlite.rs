use diesel::{Connection, SqliteConnection};

use crate::{configuration::settings::Setting, error::error::ApiError};

pub fn establish_connection() -> SqliteConnection {
    let database_url = Setting::DatabaseUrl.get();
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn check_sqlite_status() -> Result<String, ApiError> {
    let database_url = Setting::DatabaseUrl.get();
    match SqliteConnection::establish(&database_url).is_ok() {
        true => Ok("SQLite ready.".to_string()),
        false => Err(ApiError::Generic500Error(
            "Something went wrong with SQLite.".to_string(),
        )),
    }
}

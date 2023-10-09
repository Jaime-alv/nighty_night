use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::configuration::settings::Setting;

pub fn establish_connection() -> PgConnection {
    let database_url = Setting::DatabaseUrl.get();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
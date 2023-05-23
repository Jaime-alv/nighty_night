use diesel::{Connection, SqliteConnection};

use crate::configuration::settings::database_url;

pub fn establish_connection() -> SqliteConnection {
    let database_url = database_url();
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod assertion_common;
pub mod user_common;

pub mod cte {
    pub const VALUE_NONE: &'static str = "Some value expected, found None";
    pub const DB_ERROR: &'static str = "Error connecting database";
    pub const NO_USER_ERROR: &'static str = "No user found.";
}

pub mod initialiser {
    use std::path::Path;

    pub fn init() {
        dotenvy::from_path(Path::new("./key/local.env")).unwrap();
    }
}

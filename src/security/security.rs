use bcrypt::{hash, DEFAULT_COST, verify};

pub fn hash_password<T>(password: T) -> String
where
    T: Into<String>,
{
    hash(password.into(), DEFAULT_COST).unwrap()
}

pub fn verify_password(password: String, input_password: &str) -> bool {
    verify(input_password, password.as_ref()).unwrap_or(false)
}
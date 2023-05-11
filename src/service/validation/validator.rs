/// Checks if all fields have some value.
pub fn validate_fields(fields: &[&str]) -> bool {
    fields.iter().all(|f| !f.is_empty())
}


pub fn valid_password<T: Into<String>>(password: T) -> bool {
    password.into().len() < 4
}

#[cfg(test)]
mod test_validator {
    use super::*;

    #[test]
    fn test_validate_fields() {
        assert!(validate_fields(&[&"d", &"d"]));
        assert!(!validate_fields(&[&"", &"d"]));
        assert!(!validate_fields(&[&"", &""]));
    }

    #[test]
    fn test_with_vector() {
        let v = vec!["d", "d"];
        assert!(validate_fields(&v));
    }
}

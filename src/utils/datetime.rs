use chrono::{NaiveDateTime, Utc};

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn to_date(date: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")
}

#[cfg(test)]
mod test_timestamp {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(
            to_date("2023-03-23 23:31:22").unwrap(),
            NaiveDate::from_ymd_opt(2023, 3, 23)
                .unwrap()
                .and_hms_opt(23, 31, 22)
                .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_bad_format() {
        to_date("2021-02-28 23:31:22.2334").unwrap();

    }
}

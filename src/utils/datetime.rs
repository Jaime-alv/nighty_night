use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn to_date_time(date: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").expect("Date format should be like: %Y-%m-%d %H:%M")
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_time(date: NaiveTime) -> String {
    date.format("%H:%M").to_string()
}

pub async fn date_is_higher(date: NaiveDateTime, other_date: NaiveDateTime) -> bool {
    if let std::cmp::Ordering::Greater = other_date.cmp(&date) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod test_timestamp {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(
            to_date_time("2023-03-23 23:31"),
            NaiveDate::from_ymd_opt(2023, 3, 23)
                .unwrap()
                .and_hms_opt(23, 31, 00)
                .unwrap()
        );
    }

    #[tokio::test]
    async fn test_compare_dates() {
        assert!(
            date_is_higher(
                to_date_time("2023-03-23 23:31"),
                to_date_time("2023-03-23 23:32")
            )
            .await
        );
        assert!(
            !date_is_higher(
                to_date_time("2023-03-23 23:33"),
                to_date_time("2023-03-23 23:32")
            )
            .await
        );
    }
}

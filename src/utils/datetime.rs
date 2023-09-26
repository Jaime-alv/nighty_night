use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local()
}

pub fn today() -> NaiveDate {
    now().date()
}

pub fn convert_to_date_time(date_time: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M")
}

pub fn convert_to_date(date: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(&date, "%Y-%m-%d")
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_time(time: NaiveTime) -> String {
    time.format("%H:%M").to_string()
}

/// From date is a past date of the other date (future_date)
pub fn date_time_is_lower_than_other_date(
    from_date: NaiveDateTime,
    future_date: NaiveDateTime,
) -> bool {
    if let std::cmp::Ordering::Less = future_date.cmp(&from_date) {
        false
    } else {
        true
    }
}

pub fn format_duration(elapsed_minutes: i64) -> String {
    let hours: u32 = (elapsed_minutes / 60).try_into().unwrap();
    let minutes: u32 = (elapsed_minutes % 60).try_into().unwrap();
    format!("{hours:0>2}:{minutes:0>2}")
    // NaiveTime::from_hms_opt(hours, minutes, 0).expect("Invalid time format.")
}

/// Iter between to dates, excluding upper date.
///
/// From 2023-06-06 To 2023-06-10:
///
/// Vec \[2023-06-06, 2023-06-07, 2023-06-08, 2023-06-09\]
pub fn iter_between_two_dates(from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
    let days: usize = ((to - from).num_days()).try_into().unwrap();
    from.iter_days().take(days).collect()
}

/// Check an input string and convert to an Option<date>
///
/// If date is a past of baseline_date returns None.
pub fn parse_string_to_optional_date(
    baseline_date: NaiveDateTime,
    string_timestamp: &str,
) -> Option<NaiveDateTime> {
    match string_timestamp {
        "null" => None,
        _ => {
            let date: Option<NaiveDateTime> = match convert_to_date_time(&string_timestamp) {
                Ok(date_value) => {
                    if date_time_is_lower_than_other_date(baseline_date, date_value) {
                        Some(date_value)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };
            date
        }
    }
}

#[cfg(test)]
mod test_timestamp {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_parse_option_string() {
        fn into_some(date: &str) -> Option<NaiveDateTime> {
            Some(convert_to_date_time(date).unwrap())
        }
        let baseline_date = convert_to_date_time("2023-09-22 13:00").unwrap();
        let example_date_one = "2023-09-24 14:00";
        let example_date_two = "2023-09-24 14:aa";
        let example_date_three = "2023-09-21 14:00";
        let example_date_four = "null";
        assert_eq!(
            parse_string_to_optional_date(baseline_date, example_date_one),
            into_some(example_date_one)
        );
        assert_eq!(
            parse_string_to_optional_date(baseline_date, example_date_two),
            None
        );
        assert_eq!(
            parse_string_to_optional_date(baseline_date, example_date_three),
            None
        );
        assert_eq!(
            parse_string_to_optional_date(baseline_date, example_date_four),
            None
        );
    }

    #[test]
    fn test_date() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2023, 6, 7).unwrap(),
            convert_to_date("2023-06-07").unwrap()
        );
    }

    #[test]
    fn test_parse_date() {
        assert_eq!(
            convert_to_date_time("2023-03-23 23:31").unwrap(),
            NaiveDate::from_ymd_opt(2023, 3, 23)
                .unwrap()
                .and_hms_opt(23, 31, 00)
                .unwrap()
        );
    }

    #[test]
    fn test_compare_dates() {
        assert!(date_time_is_lower_than_other_date(
            convert_to_date_time("2023-03-23 23:31").unwrap(),
            convert_to_date_time("2023-03-23 23:32").unwrap()
        ));
        assert!(!date_time_is_lower_than_other_date(
            convert_to_date_time("2023-03-23 23:33").unwrap(),
            convert_to_date_time("2023-03-23 23:32").unwrap()
        ));
    }

    #[test]
    fn test_duration() {
        assert_eq!(
            format_duration(60),
            format_time(NaiveTime::from_hms_opt(1, 0, 0).unwrap())
        );
        assert_eq!(
            format_duration(90),
            format_time(NaiveTime::from_hms_opt(1, 30, 0).unwrap())
        );
        assert_eq!(
            format_duration(45),
            format_time(NaiveTime::from_hms_opt(0, 45, 0).unwrap())
        );
        assert_eq!(
            format_duration(0),
            format_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        );
    }

    #[test]
    fn test_iter_days() {
        let d1 = NaiveDate::from_ymd_opt(2023, 6, 6).unwrap();
        let d2 = NaiveDate::from_ymd_opt(2023, 6, 9).unwrap();
        let week = Vec::from([
            NaiveDate::from_ymd_opt(2023, 6, 6).unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 7).unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 8).unwrap(),
            NaiveDate::from_ymd_opt(2023, 6, 9).unwrap(),
        ]);
        assert_eq!(iter_between_two_dates(d1, d2), week);
    }
}

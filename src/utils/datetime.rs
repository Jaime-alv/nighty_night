use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local()
}

pub fn to_date_time(date_time: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M")
        .expect("Date format should be like: %Y-%m-%d %H:%M")
}

pub fn to_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Date format should be like: %Y-%m-%d")
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_time(time: NaiveTime) -> String {
    time.format("%H:%M").to_string()
}

pub async fn date_is_higher(date: NaiveDateTime, other_date: NaiveDateTime) -> bool {
    if let std::cmp::Ordering::Greater = other_date.cmp(&date) {
        true
    } else {
        false
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
pub fn iter_between_to_dates(from: NaiveDate, to: NaiveDate) -> Vec<NaiveDate> {
    let days: usize = ((to - from).num_days()).try_into().unwrap();
    dbg!(days);
    from.iter_days()
        .take(days)
        .collect()
}

#[cfg(test)]
mod test_timestamp {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_date() {
        assert_eq!(
            NaiveDate::from_ymd_opt(2023, 6, 7).unwrap(),
            to_date("2023-06-07")
        )
    }

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
        assert_eq!(iter_between_to_dates(d1, d2), week);
    }
}

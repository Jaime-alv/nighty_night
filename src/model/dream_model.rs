use chrono::{NaiveDateTime, NaiveTime};
use diesel::{data_types::PgInterval, Identifiable, Insertable, Queryable};

use crate::{
    schema::dreams,
    utils::datetime::{duration_as_time, format_date, format_time, now},
};

#[derive(Queryable, Identifiable)]
#[diesel(table_name = dreams)]
pub struct Dream {
    id: i32,
    baby_id: i32,
    from_date: NaiveDateTime,
    to_date: Option<NaiveDateTime>,
    elapsed: Option<PgInterval>,
}

impl Dream {
    pub fn from_date(&self) -> NaiveDateTime {
        self.from_date
    }

    pub fn formatted_from_date(&self) -> String {
        format_date(self.from_date.date())
    }

    pub fn formatted_to_date(&self) -> String {
        match self.to_date {
            Some(d) => format_date(d.date()),
            None => "-".to_string(),
        }
    }

    pub fn formatted_from_time(&self) -> String {
        format_time(self.from_date.time())
    }

    pub fn formatted_to_time(&self) -> String {
        match self.to_date {
            Some(d) => format_time(d.time()),
            None => "-".to_string(),
        }
    }

    pub fn to_date(&self) -> NaiveDateTime {
        match self.to_date {
            Some(date) => date,
            None => NaiveDateTime::default(),
        }
    }

    pub fn formatted_elapsed(&self) -> String {
        format_time(self.elapsed())
    }

    pub fn elapsed(&self) -> NaiveTime {
        let time = match self.elapsed {
            Some(time) => time.microseconds,
            None => PgInterval::new(0, 0, 0).microseconds,
        };
        let minutes = time / 60000000;
        duration_as_time(minutes)
    }

    pub(crate) fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Insertable)]
#[diesel(table_name = dreams)]
pub struct InsertableDream {
    baby_id: i32,
    from_date: Option<NaiveDateTime>,
    to_date: Option<NaiveDateTime>,
}

impl InsertableDream {
    pub fn new(
        baby_id: i32,
        from_date: Option<NaiveDateTime>,
        to_date: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            baby_id,
            from_date,
            to_date,
        }
    }

    pub fn baby_id(&self) -> i32 {
        self.baby_id
    }

    pub fn to_date(&self) -> NaiveDateTime {
        self.to_date.unwrap_or(now())
    }
}

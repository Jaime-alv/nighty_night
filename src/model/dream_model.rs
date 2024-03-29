use chrono::{Duration, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable};

use crate::{
    data::dream_dto::InputDreamDto,
    schema::dreams,
    utils::datetime::{
        convert_to_date_time, format_date, format_duration, format_time, now,
        parse_string_to_optional_date,
    },
};

#[derive(Queryable, Identifiable, Clone)]
#[diesel(table_name = dreams)]
pub struct Dream {
    id: i32,
    baby_id: i32,
    from_date: NaiveDateTime,
    to_date: Option<NaiveDateTime>,
}

impl Dream {
    pub fn new(
        id: i32,
        baby_id: i32,
        from_date: NaiveDateTime,
        to_date: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            baby_id,
            from_date,
            to_date,
        }
    }

    pub fn baby_id(&self) -> i32 {
        self.baby_id
    }

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

    pub fn to_date(&self) -> Option<NaiveDateTime> {
        // match self.to_date {
        //     Some(date) => date,
        //     None => NaiveDateTime::default(),
        // }
        self.to_date
    }

    pub fn formatted_elapsed(&self) -> String {
        let time = match self.to_date {
            Some(time) => time - self.from_date,
            None => Duration::minutes(0),
        };
        format_duration(time.num_minutes())
    }

    pub fn elapsed(&self) -> Duration {
        match self.to_date {
            Some(to) => to - self.from_date,
            None => Duration::minutes(0),
        }
    }

    pub(crate) fn id(&self) -> i32 {
        self.id
    }

    pub fn update_dream(&self, dream_record: InputDreamDto) -> Self {
        let new_from_date = match dream_record.from_date {
            Some(value) => convert_to_date_time(&value).unwrap_or(self.from_date),
            None => self.from_date,
        };
        let new_to_date = match dream_record.to_date {
            Some(to_time_value) => parse_string_to_optional_date(new_from_date, &to_time_value),
            None => self.to_date,
        };
        Self {
            from_date: new_from_date,
            to_date: new_to_date,
            ..self.clone()
        }
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

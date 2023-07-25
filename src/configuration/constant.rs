const LAST_DAYS_DEFAULT: u32 = 7;
const WEIGHT_LAST_DAYS_DEFAULT: u32 = 7;
const MAX_DAYS: u32 = 365;
const DEFAULT_RECORDS_PER_PAGE: u32 = 100;
pub enum GlobalCte {
    /// How many days, by default, get from the db.
    LastDaysCte,
    /// Max difference between dates to get from db.
    DaysOutOfBoundsCte,
    /// How many records show per page by default.
    RecordsPerPage,
    WeightLastDaysDefault,
}

impl GlobalCte {
    pub fn get(&self) -> u32 {
        match self {
            GlobalCte::LastDaysCte => LAST_DAYS_DEFAULT,
            GlobalCte::DaysOutOfBoundsCte => MAX_DAYS,
            GlobalCte::RecordsPerPage => DEFAULT_RECORDS_PER_PAGE,
            GlobalCte::WeightLastDaysDefault => WEIGHT_LAST_DAYS_DEFAULT,
        }
    }
}

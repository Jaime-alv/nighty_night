const LAST_DAYS_DEFAULT: u32 = 7;
const WEIGHT_LAST_DAYS_DEFAULT: u32 = 30;
const DEFAULT_RECORDS_PER_PAGE: u32 = 100;
const MAX_PAGINATION_THRESHOLD: u32 = 365;
const DELETE_ACCOUNT: u32 = 180;
const DEFAULT_ANONYMOUS_ID: u32 = 1;
pub enum GlobalCte {
    /// How many days, by default, get from the db.
    LastDaysCte,
    /// How many records show per page by default.
    RecordsPerPage,
    /// Max records allowed in pagination.
    MaxPaginationThreshold,
    WeightLastDaysDefault,
    DeleteAccount,
    DefaultAnonymousID
}

impl GlobalCte {
    pub fn get(&self) -> u32 {
        match self {
            GlobalCte::LastDaysCte => LAST_DAYS_DEFAULT,
            GlobalCte::RecordsPerPage => DEFAULT_RECORDS_PER_PAGE,
            GlobalCte::WeightLastDaysDefault => WEIGHT_LAST_DAYS_DEFAULT,
            GlobalCte::MaxPaginationThreshold => MAX_PAGINATION_THRESHOLD,
            GlobalCte::DeleteAccount => DELETE_ACCOUNT,
            GlobalCte::DefaultAnonymousID => DEFAULT_ANONYMOUS_ID,
        }
    }
}

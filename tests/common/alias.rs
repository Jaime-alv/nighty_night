use nighty_night::{response::{response::RecordResponse, error::ApiError}, data::common_structure::SessionDto};

pub type ResponseSession = Result<RecordResponse<SessionDto>, ApiError>;
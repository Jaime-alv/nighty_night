use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        dream_dto::DreamData,
    },
    model::dream_model::Dream,
};

impl From<Dream> for BasicDataStruct<DreamData> {
    fn from(dream: Dream) -> Self {
        let attributes = DreamData {
            from_date: dream.formatted_from_date(),
            from_time: dream.formatted_from_time(),
            to_date: dream.formatted_to_date(),
            to_time: dream.formatted_to_time(),
            elapsed: dream.formatted_elapsed(),
        };
        BasicDataStruct::new(dream.id(), DataType::Dream, attributes)
    }
}

use crate::{
    data::dream_dto::{DreamAttributes, DreamDto},
    model::dream_model::Dream,
};

use super::data_type::DataType;

impl From<Dream> for DreamDto {
    fn from(dream: Dream) -> Self {
        let attr = DreamAttributes {
            from_date: dream.formatted_from_date(),
            from_time: dream.formatted_from_time(),
            to_date: dream.formatted_to_date(),
            to_time: dream.formatted_to_time(),
            elapsed: dream.formatted_elapsed(),
        };
        DreamDto {
            id: dream.id(),
            r#type: DataType::Dream.get(),
            attributes: attr,
        }
    }
}

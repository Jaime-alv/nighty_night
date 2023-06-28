use crate::{
    data::dream_dto::{DreamDto},
    model::dream_model::{Dream},
};

impl From<Dream> for DreamDto {
    fn from(dream: Dream) -> Self {
        DreamDto {
            id: dream.id(),
            from_date: dream.formatted_from_date(),
            from_time: dream.formatted_from_time(),
            to_date: dream.formatted_to_date(),
            to_time: dream.formatted_to_time(),
            elapsed: dream.formatted_elapsed()
        }
    }
}

pub fn from_dream_to_dream_dto_vector(dreams: Vec<Dream>) -> Vec<DreamDto> {
    dreams
        .into_iter()
        .map(|dream| DreamDto::from(dream))
        .collect()
}

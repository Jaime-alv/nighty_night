use crate::{data::weight_dto::WeightDto, model::weight_model::Weight};

impl From<Weight> for WeightDto {
    fn from(value: Weight) -> Self {
        WeightDto {
            id: value.id(),
            date: value.formatted_date(),
            value: value.value(),
        }
    }
}
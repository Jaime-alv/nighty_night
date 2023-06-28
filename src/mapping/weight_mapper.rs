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

pub fn from_weight_to_weight_dto_vector(weights: Vec<Weight>) -> Vec<WeightDto> {
    weights.into_iter().map(|value| value.into()).collect()
}

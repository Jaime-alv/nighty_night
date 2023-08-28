use crate::{
    data::weight_dto::{WeightAttributes, WeightDto},
    model::weight_model::Weight,
};

use super::data_type::DataType;

impl From<Weight> for WeightDto {
    fn from(value: Weight) -> Self {
        let attr = WeightAttributes {
            date: value.formatted_date(),
            value: value.value(),
        };
        WeightDto {
            id: value.id(),
            r#type: DataType::Weight.get(),
            attributes: attr,
        }
    }
}

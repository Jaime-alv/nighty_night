use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        weight_dto::WeightData,
    },
    model::weight_model::Weight,
};

impl From<Weight> for BasicDataStruct<WeightData> {
    fn from(value: Weight) -> Self {
        let attributes = WeightData {
            date: value.formatted_date(),
            value: value.value(),
        };
        BasicDataStruct::new(value.id(), DataType::Weight, attributes)
    }
}

use crate::{
    data::{
        baby_dto::BabyData,
        common_structure::{BasicDataStruct, DataType},
    },
    model::baby_model::Baby,
};

impl From<Baby> for BasicDataStruct<BabyData> {
    fn from(baby: Baby) -> Self {
        let attributes = BabyData {
            unique_id: baby.unique_id(),
            name: baby.name(),
            birthdate: baby.formatted_birthdate(),
        };
        BasicDataStruct::new(baby.id(), DataType::Baby, attributes)
    }
}

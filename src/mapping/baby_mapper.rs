use crate::{
    data::baby_dto::{BabyAttributes, BabyDto},
    model::baby_model::Baby,
};

use super::data_type::DataType;

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        let attr = BabyAttributes {
            name: baby.name(),
            birthdate: baby.formatted_birthdate(),
        };
        BabyDto {
            id: baby.id(),
            r#type: DataType::Baby.get(),
            attributes: attr,
        }
    }
}

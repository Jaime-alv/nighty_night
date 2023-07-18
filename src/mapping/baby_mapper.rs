use crate::{data::baby_dto::BabyDto, model::baby_model::Baby};

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        BabyDto {
            id: baby.id(),
            name: baby.name(),
            birthdate: baby.formatted_birthdate(),
        }
    }
}

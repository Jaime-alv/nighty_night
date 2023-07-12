use crate::{
    data::baby_dto::{BabyDto, NewBabyDto},
    model::baby_model::{Baby, InsertableBaby},
    utils::datetime::convert_to_date,
};

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        BabyDto {
            id: baby.id(),
            name: baby.name(),
            birthdate: baby.formatted_birthdate(),
        }
    }
}

impl From<NewBabyDto> for InsertableBaby {
    fn from(baby: NewBabyDto) -> Self {
        InsertableBaby::new(baby.name, convert_to_date(&baby.birthdate).unwrap())
    }
}

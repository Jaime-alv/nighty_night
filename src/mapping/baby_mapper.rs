use crate::{
    data::baby_dto::{BabyDto, NewBabyDto},
    model::baby_model::{Baby, InsertableBaby},
    utils::datetime::to_date,
};

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        BabyDto {
            id: baby.id(),
            name: baby.name(),
            birthdate: baby.formatted_birthdate()
        }
    }
}

pub async fn babies_to_babies_dto(babies: Vec<Baby>) -> Vec<BabyDto> {
    babies.into_iter().map(|b| BabyDto::from(b)).collect()
}

impl From<NewBabyDto> for InsertableBaby {
    fn from(baby: NewBabyDto) -> Self {
        InsertableBaby::new(baby.name, to_date(&baby.birthdate))
    }
}

use crate::{model::baby_model::Baby, data::baby_dto::BabyDto};

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        BabyDto::new(baby.name())
    }
}

pub fn babies_to_babies_dto(babies: Vec<Baby>) -> Vec<BabyDto> {
    babies.into_iter().map(|b| BabyDto::from(b)).collect()
}
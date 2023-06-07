use crate::{data::baby_dto::BabyDto, model::baby_model::Baby};

impl From<Baby> for BabyDto {
    fn from(baby: Baby) -> Self {
        BabyDto { id: baby.id(), name: baby.name() }
    }
}

pub async fn babies_to_babies_dto(babies: Vec<Baby>) -> Vec<BabyDto> {
    babies.into_iter().map(|b| BabyDto::from(b)).collect()
}

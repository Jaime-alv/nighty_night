use crate::{
    data::{dream_dto::DreamSummaryDto, meal_dto::MealSummaryDto},
    model::summary_model::{DreamSummary, MealSummary},
};

impl From<DreamSummary> for DreamSummaryDto {
    fn from(dream: DreamSummary) -> Self {
        DreamSummaryDto {
            date: dream.formatted_date(),
            summary: dream.formatted_summary(),
        }
    }
}

impl From<MealSummary> for MealSummaryDto {
    fn from(meal: MealSummary) -> Self {
        MealSummaryDto {
            date: meal.formatted_date(),
            total_feedings: meal.total_feedings(),
            nursing_time: meal.formatted_nursing_time(),
            formula: meal.formula(),
        }
    }
}

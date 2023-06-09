use crate::{data::meal_dto::MealDto, model::meals_model::Meal};

impl From<Meal> for MealDto {
    fn from(meal: Meal) -> Self {
        MealDto {
            id: meal.id(),
            date: meal.formatted_date(),
            time: meal.formatted_time(),
            quantity: meal.quantity(),
            elapsed: meal.formatted_elapsed(),
        }
    }
}

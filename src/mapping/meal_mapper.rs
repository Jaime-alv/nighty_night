use crate::{data::meal_dto::MealDto, model::meals_model::Meal};

impl From<Meal> for MealDto {
    fn from(meal: Meal) -> Self {
        MealDto {
            date: meal.formatted_date(),
            time: meal.formatted_time(),
            quantity: meal.quantity(),
            elapsed: meal.elapsed(),
        }
    }
}

pub async fn from_meal_to_meal_dto_vector(meals: Vec<Meal>) -> Vec<MealDto> {
    meals.into_iter().map(|meal| MealDto::from(meal)).collect()
}

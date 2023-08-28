use crate::{
    data::meal_dto::{MealAttributes, MealDto},
    model::meals_model::Meal,
};

use super::data_type::DataType;

impl From<Meal> for MealDto {
    fn from(meal: Meal) -> Self {
        let attributes = MealAttributes {
            date: meal.formatted_date(),
            time: meal.formatted_time(),
            quantity: meal.formatted_quantity(),
            elapsed: meal.formatted_elapsed(),
        };
        MealDto {
            id: meal.id(),
            attributes,
            r#type: DataType::Meal.get(),
        }
    }
}

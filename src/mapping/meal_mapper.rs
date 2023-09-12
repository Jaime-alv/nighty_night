use crate::{
    data::{
        common_structure::{BasicDataStruct, DataType},
        meal_dto::MealData,
    },
    model::meals_model::Meal,
};

impl From<Meal> for BasicDataStruct<MealData> {
    fn from(meal: Meal) -> Self {
        let attributes = MealData {
            date: meal.formatted_date(),
            time: meal.formatted_time(),
            quantity: meal.formatted_quantity(),
            elapsed: meal.formatted_elapsed(),
        };
        BasicDataStruct::new(meal.id(), DataType::Meal, attributes)
    }
}

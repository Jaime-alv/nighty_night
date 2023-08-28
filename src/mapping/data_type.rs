pub(super) enum DataType {
    User,
    Baby,
    Dream,
    Meal,
    Weight,
}

impl DataType {
    pub fn get<'a>(self) -> &'a str {
        match self {
            DataType::User => "User",
            DataType::Baby => "Baby",
            DataType::Dream => "dream",
            DataType::Meal => "meal",
            DataType::Weight => "weight",
        }
    }
}

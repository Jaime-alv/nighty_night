from datetime import date, datetime

from . import const
from . import utils_script as us


class MealDay:
    default_date: date = datetime.min.date()

    def __init__(
        self,
        iso_date: date = default_date,
        meals: list[str] = [],
        year: int = const.YEAR,
    ) -> None:
        day_str: str = us.format_day_sql(iso_date)
        self.year = year
        self.queries: list[str] = [day_str]
        self.iso_date = iso_date
        self.meals = meals

    def set_fields(self, records: list[str], year: int = const.YEAR) -> "MealDay":
        day, meals = us.split_date_and_data(records, self.year)
        return MealDay(day, meals, year)

    @staticmethod
    def format_query(id: int, timestamp: datetime, quantity: int) -> str:
        values: str = f"{id}, '{timestamp}', {quantity}"
        return f"INSERT INTO meals (baby_id, date, quantity) VALUES ({values});"

    def get_queries(self) -> list[str]:
        for meal in self.meals:
            time, quantity = meal.split()[0], int(meal.split()[1])
            timestamp: datetime = us.create_timestamp(self.iso_date, time)
            query: str = self.format_query(const.BABY_ID, timestamp, quantity)
            self.queries.append(query)
        return self.queries

    def __repr__(self) -> str:
        dbg: str = (
            f"Date: {self.iso_date}\nMeals: {self.meals}\nQueries: {self.get_queries()}"
        )
        return dbg


def split_list(original_list: list[str]) -> list[MealDay]:
    explode: list[list[str]] = us.explode_list(original_list, "")
    return [MealDay().set_fields(day) for day in explode]


def compose_meal_list(data: list[MealDay]) -> list[str]:
    return us.flatten_nested_list([meals.get_queries() for meals in data])


def compose_meals_queries(original_list: list[str]) -> list[str]:
    return compose_meal_list(split_list(original_list))


def read_process_and_save_eat_file(file: str) -> None:
    # Read data from file
    input_folder: str = f"{const.PATH}{const.INPUT}{file}"
    output_folder: str = f"{const.PATH}{const.OUTPUT}{file}.sql"
    raw_data = us.open_file(input_folder)
    # Split records and create entities
    split_data = split_list(raw_data)
    # Iterate over records
    queries = compose_meal_list(split_data)
    # Save queries to disk
    us.save_to_file(output_folder, queries)

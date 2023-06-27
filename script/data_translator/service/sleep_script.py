from datetime import date, datetime, timedelta

from . import const
from . import utils_script as us


class DreamDay:
    default_date: date = datetime.min.date()

    def __init__(
        self,
        iso_date: date = default_date,
        dreams: list[str] = [],
        year: int = const.YEAR,
    ) -> None:
        day_str: str = us.format_day_sql(iso_date)
        self.year = year
        self.queries: list[str] = [day_str]
        self.iso_date = iso_date
        self.dreams = dreams

    def set_fields(self, records: list[str], year: int = const.YEAR) -> "DreamDay":
        day, dreams = us.split_date_and_data(records, year)
        return DreamDay(day, dreams, year)

    @staticmethod
    def format_query(id: int, from_time: datetime, to_time: str | datetime) -> str:
        values: str = f"{id}, '{from_time}', '{to_time}'"
        return f"INSERT INTO dreams (baby_id, from_date, to_date) VALUES ({values});"

    def get_queries(self) -> list[str]:
        for dream in self.dreams:
            data: list[str] = dream.split()
            from_time: datetime = us.create_timestamp(self.iso_date, data[0])
            to_time: datetime = us.create_timestamp(self.iso_date, data[1])
            if (to_time - from_time).days < 0:
                to_time += timedelta(days=1)

            query: str = self.format_query(const.BABY_ID, from_time, to_time)
            self.queries.append(query)
        return self.queries

    def __repr__(self) -> str:
        dbg: str = f"Date: {self.iso_date}\nMeals: {self.dreams}\nQueries: {self.get_queries()}"
        return dbg


def split_list(original_list: list[str]) -> list[DreamDay]:
    explode: list[list[str]] = us.explode_list(original_list, "")
    return [DreamDay().set_fields(day) for day in explode]


def compose_dream_list(data: list[DreamDay]) -> list[str]:
    return us.flatten_nested_list([dreams.get_queries() for dreams in data])


def read_process_and_save_dream_file(file: str) -> None:
    # Read data from file
    input_folder: str = f"{const.PATH}{const.INPUT}{file}"
    output_folder: str = f"{const.PATH}{const.OUTPUT}{file}.sql"
    raw_data = us.open_file(input_folder)
    # Split records and create entities
    split_data = split_list(raw_data)
    # Iterate over records
    queries = compose_dream_list(split_data)
    # Save queries to disk
    us.save_to_file(output_folder, queries)

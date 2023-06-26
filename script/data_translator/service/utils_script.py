import itertools
from datetime import date, datetime
from pathlib import Path


def open_file(file_path: str) -> list[str]:
    if not Path(file_path).exists():
        create_files(file_path)
    with Path(file_path).open("r", encoding='ASCII') as open_file:
        parsed = open_file.readlines()
        clean_data = [e.strip("\n") for e in parsed]
        return clean_data


def save_to_file(file_path: str, queries: list[str]) -> None:
    if not Path(file_path).exists():
        create_files(file_path)
    with Path(file_path).open("w", encoding='ASCII') as clear:
        clear.write("\n".join(queries))


def create_files(file_path: str) -> None:
    # Path(file_path).mkdir(parents=True, exist_ok=True)
    Path(file_path).touch(exist_ok=True)


def create_date(year: int, month: int, day: int) -> date:
    iso_date: date = datetime.strptime(f"{year}-{month}-{day}", "%Y-%m-%d").date()
    return iso_date


def create_timestamp(date: date, time: str) -> datetime:
    iso_time = datetime.strptime(time, "%H:%M").time()
    return datetime.combine(date, iso_time)


def flatten_nested_list(data: list[list[str]]) -> list[str]:
    return [item for sublist in data for item in sublist]


def explode_list(data: list[str], val: str) -> list[list[str]]:
    return [
        list(group) for k, group in itertools.groupby(data, lambda x: x == val) if not k
    ]


def parse_date(line: str, year: int) -> date:
    unwrap = line.split("-")
    month, day = int(unwrap[1]), int(unwrap[0])
    return create_date(year, month, day)


def split_date_and_data(raw_data: list[str], year: int) -> tuple[date, list[str]]:
    iso_date: date = parse_date(raw_data[0], year)
    data: list[str] = raw_data[1::]
    return iso_date, data


def format_day_sql(day: date) -> str:
    return f"-- {day.day:02}-{day.month:02}-{day.year}"

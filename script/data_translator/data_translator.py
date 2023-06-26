from service.eat_script import read_process_and_save_eat_file
from service.logger import error, info, set_logger
from service.sleep_script import read_process_and_save_dream_file

text: str = """
Raw data to SQL translator:
1) Eat file
2) Sleep file
3) Both files
"""


def parse_response(user_input: str) -> None:
    ok: str = "File saved!"
    try:
        option: int = int(user_input)
    except ValueError:
        return error("Error! Invalid character")
    match option:
        case 1:
            read_process_and_save_eat_file("eat")
            return info(ok)
        case 2:
            read_process_and_save_dream_file("sleep")
            return info(ok)
        case 3:
            read_process_and_save_eat_file("eat")
            read_process_and_save_dream_file("sleep")
            return info(ok)
        case _:
            return error("Error!")


if __name__ == "__main__":
    set_logger()
    info("Initialize script.")
    print(text)
    user_input: str = input("Option: ")
    parse_response(user_input)

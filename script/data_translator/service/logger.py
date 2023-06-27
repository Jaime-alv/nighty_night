import logging

from . import const


def set_logger(level: int = const.LEVEL) -> None:
    return logging.basicConfig(
        level=level, format="%(asctime)s - %(levelname)s - %(message)s"
    )


def info(msg: str) -> None:
    logging.info(msg)


def error(msg: str) -> None:
    logging.error(msg)


def dbg(msg: str) -> None:
    logging.debug(msg)

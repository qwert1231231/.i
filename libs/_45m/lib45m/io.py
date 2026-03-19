from __future__ import annotations

from enum import IntEnum


class io_error_t(IntEnum):
    IO_OK = 0
    IO_EOF = 1
    IO_INVALID = 2
    IO_IO_ERROR = 3
    IO_UNKNOWN = 4


_last: io_error_t = io_error_t.IO_OK


def io_last_error() -> io_error_t:
    return _last


def io_error_str(e: io_error_t) -> str:
    return e.name


def io_print(s: str) -> int:
    global _last
    try:
        print(s, end="")
        _last = io_error_t.IO_OK
        return 1
    except Exception:
        _last = io_error_t.IO_IO_ERROR
        return 0


def io_println(s: str) -> int:
    global _last
    try:
        print(s)
        _last = io_error_t.IO_OK
        return 1
    except Exception:
        _last = io_error_t.IO_IO_ERROR
        return 0


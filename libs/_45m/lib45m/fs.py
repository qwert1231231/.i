from __future__ import annotations

from enum import IntEnum
from pathlib import Path


class fs_error_t(IntEnum):
    FS_OK = 0
    FS_NOT_FOUND = 1
    FS_PERMISSION_DENIED = 2
    FS_ALREADY_EXISTS = 3
    FS_INVALID_PATH = 4
    FS_IO_ERROR = 5
    FS_UNKNOWN = 6


_last: fs_error_t = fs_error_t.FS_OK


def fs_last_error() -> fs_error_t:
    return _last


def fs_error_str(e: fs_error_t) -> str:
    return e.name


def _set(e: fs_error_t) -> None:
    global _last
    _last = e


def fs_write_text(path: str, text: str) -> int:
    try:
        p = Path(path)
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
        _set(fs_error_t.FS_OK)
        return 1
    except FileNotFoundError:
        _set(fs_error_t.FS_NOT_FOUND)
        return 0
    except PermissionError:
        _set(fs_error_t.FS_PERMISSION_DENIED)
        return 0
    except OSError:
        _set(fs_error_t.FS_IO_ERROR)
        return 0
    except Exception:
        _set(fs_error_t.FS_UNKNOWN)
        return 0


def fs_append_text(path: str, text: str) -> int:
    try:
        p = Path(path)
        p.parent.mkdir(parents=True, exist_ok=True)
        with p.open("a", encoding="utf-8") as f:
            f.write(text)
        _set(fs_error_t.FS_OK)
        return 1
    except FileNotFoundError:
        _set(fs_error_t.FS_NOT_FOUND)
        return 0
    except PermissionError:
        _set(fs_error_t.FS_PERMISSION_DENIED)
        return 0
    except OSError:
        _set(fs_error_t.FS_IO_ERROR)
        return 0
    except Exception:
        _set(fs_error_t.FS_UNKNOWN)
        return 0


def fs_read_text_value(path: str) -> str:
    try:
        p = Path(path)
        data = p.read_text(encoding="utf-8")
        _set(fs_error_t.FS_OK)
        return data
    except FileNotFoundError:
        _set(fs_error_t.FS_NOT_FOUND)
        raise
    except PermissionError:
        _set(fs_error_t.FS_PERMISSION_DENIED)
        raise
    except OSError:
        _set(fs_error_t.FS_IO_ERROR)
        raise
    except Exception:
        _set(fs_error_t.FS_UNKNOWN)
        raise


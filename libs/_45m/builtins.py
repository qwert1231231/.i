from __future__ import annotations

from pathlib import Path
from typing import Any, Callable, Dict, Optional

from libs._45m.lib45m import fs, io
try:
    # Optional: generated bulk functions (10k+)
    from libs._45m import generated_functions
except Exception:  # pragma: no cover
    generated_functions = None

# All 45m functions are automatically available globally (C-style names).
# Each callable receives (project_root, *args) so relative file paths can be resolved.


Fn = Callable[..., Any]


_FUNCTIONS: Dict[str, Fn] = {
    # io
    "io_print": lambda project_root, s="": io.io_print(str(s)),
    "io_println": lambda project_root, s="": io.io_println(str(s)),
    # fs
    "fs_write_text": lambda project_root, p, text: fs.fs_write_text(_resolve(project_root, p), str(text)),
    "fs_append_text": lambda project_root, p, text: fs.fs_append_text(_resolve(project_root, p), str(text)),
    "fs_read_text": lambda project_root, p: fs.fs_read_text_value(_resolve(project_root, p)),
}

# Register generated names (if present)
if generated_functions is not None:
    generated_functions.register(_FUNCTIONS)


def get_function(name: str) -> Optional[Fn]:
    return _FUNCTIONS.get(name)


def _resolve(project_root: Path, p: Any) -> str:
    path = str(p)
    pp = Path(path)
    if pp.is_absolute():
        return str(pp)
    return str(project_root / pp)


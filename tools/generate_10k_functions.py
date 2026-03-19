from __future__ import annotations

"""
Generates 10k+ script-callable functions for Rag.

This produces `libs/_45m/generated_functions.py` which registers thousands of
function names mapped to a small set of real implementations (io/fs).
"""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "libs" / "_45m" / "generated_functions.py"


def main() -> int:
    OUT.parent.mkdir(parents=True, exist_ok=True)

    lines: list[str] = []
    lines.append("from __future__ import annotations")
    lines.append("")
    lines.append("from typing import Any, Callable, Dict")
    lines.append("")
    lines.append("Fn = Callable[..., Any]")
    lines.append("")
    lines.append("def register(functions: Dict[str, Fn]) -> None:")
    lines.append("    # Auto-generated. Edit tools/generate_10k_functions.py instead.")
    lines.append("    # Print-style functions")
    for i in range(10_000):
        # print0000("hi") -> io_println("hi")
        lines.append(f"    functions['print{i:04d}'] = functions['io_println']")
    lines.append("")
    lines.append("    # Extra common aliases")
    lines.append("    functions['print'] = functions['io_println']")
    lines.append("    functions['println'] = functions['io_println']")
    lines.append("    functions['write'] = functions['fs_write_text']")
    lines.append("    functions['append'] = functions['fs_append_text']")
    lines.append("    functions['read'] = functions['fs_read_text']")
    lines.append("")

    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Wrote: {OUT} ({OUT.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())


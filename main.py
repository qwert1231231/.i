from __future__ import annotations

from core.repl import RagRepl


def main() -> int:
    RagRepl().run()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())


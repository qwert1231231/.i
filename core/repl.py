from __future__ import annotations

from pathlib import Path

from core.executor import execute
from core.installer import InstallError, install_package
from core.parser import RagParseError, parse_program


class RagRepl:
    def __init__(self) -> None:
        self.root = Path(__file__).resolve().parents[1]
        self.projects_dir = self.root / "projects"
        self.packages_dir = self.root / "packages"

        self.projects_dir.mkdir(parents=True, exist_ok=True)
        self.packages_dir.mkdir(parents=True, exist_ok=True)

    def run(self) -> None:
        from core.keywords import LANGUAGE_NAME

        print(f"{LANGUAGE_NAME} Terminal (Python)")
        print("Commands: run <file.i> | install <file.i> | help | exit")
        print('Tip: you can also paste `int main(void) { ... }` directly.')

        while True:
            try:
                line = input("rag> ").strip()
            except EOFError:
                print()
                return

            if not line:
                continue
            if line in {"exit", "quit"}:
                return
            if line == "help":
                print("run <file.i>        Run a .i file from /projects or by path")
                print("install <file.i>    Install a .i into /packages (shows permission prompt)")
                print("(also supports .rag for compatibility)")
                print("paste program       Start with `int main...{` then end with `}`")
                print("exit               Quit")
                continue

            # Paste mode: if the user starts typing a program, collect until braces close
            if self._looks_like_program_start(line):
                self._run_pasted_program(start_line=line)
                continue

            if line.startswith("run "):
                arg = line[4:].strip().strip('"')
                self._run_file(arg)
                continue

            if line.startswith("install "):
                arg = line[8:].strip().strip('"')
                self._install_file(arg)
                continue

            print("Unknown command. Type 'help'.")

    def _looks_like_program_start(self, line: str) -> bool:
        s = line.lstrip()
        return ("main" in s and "{" in s and ("int" in s or "void" in s)) or s.startswith("int main")

    def _run_pasted_program(self, start_line: str) -> None:
        lines = [start_line]
        depth = start_line.count("{") - start_line.count("}")

        while depth > 0:
            try:
                nxt = input("... ").rstrip("\n")
            except EOFError:
                print()
                return
            lines.append(nxt)
            depth += nxt.count("{") - nxt.count("}")

        src = "\n".join(lines)
        try:
            stmts = parse_program(src)
            code = execute(stmts, project_root=self.projects_dir)
            print(f"[exit {code}]")
        except (RagParseError, Exception) as e:
            print(f"Error: {e}")

    def _resolve_project_file(self, arg: str) -> Path:
        p = Path(arg)
        if p.is_absolute() or p.exists():
            return p
        return self.projects_dir / arg

    def _run_file(self, arg: str) -> None:
        p = self._resolve_project_file(arg)
        if not p.exists():
            print(f"File not found: {p}")
            return
        try:
            stmts = parse_program(p.read_text(encoding="utf-8"))
            code = execute(stmts, project_root=p.parent)
            print(f"[exit {code}]")
        except (RagParseError, Exception) as e:
            print(f"Error: {e}")

    def _install_file(self, arg: str) -> None:
        p = self._resolve_project_file(arg)
        try:
            dest = install_package(p, self.packages_dir)
            print(f"Installed to: {dest}")
        except InstallError as e:
            print(f"Install failed: {e}")


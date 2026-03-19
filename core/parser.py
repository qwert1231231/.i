from __future__ import annotations

import re
from dataclasses import dataclass
from typing import List, Optional

from core.keywords import KW_LET, KW_RETURN, SAY_WORDS

@dataclass
class Stmt:
    pass


@dataclass
class Call(Stmt):
    name: str
    args: List[str]


@dataclass
class Let(Stmt):
    name: str
    expr: Call


@dataclass
class Return(Stmt):
    value: int


class RagParseError(Exception):
    pass


def parse_program(src: str) -> List[Stmt]:
    """
    Minimal C-style subset:
      - int main(void) { ... }
      - statements separated by ;
      - function calls: name("string", 123)
      - let x = call(...);
      - return 0;
    """
    src = re.sub(r"//.*?$", "", src, flags=re.MULTILINE)
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.DOTALL)

    m = re.search(r"\bint\s+main\s*\([^)]*\)\s*\{", src)
    if not m:
        raise RagParseError("Expected: int main(void) { ... }")

    i = m.end()
    depth = 1
    j = i
    while j < len(src) and depth > 0:
        if src[j] == "{":
            depth += 1
        elif src[j] == "}":
            depth -= 1
        j += 1
    if depth != 0:
        raise RagParseError("Unbalanced braces in main()")

    body = src[i : j - 1]
    stmts = _split_statements(body)
    return [_parse_stmt(s) for s in stmts]


def _split_statements(block: str) -> List[str]:
    out: List[str] = []
    cur: List[str] = []
    in_str = False
    esc = False
    for ch in block:
        if in_str:
            cur.append(ch)
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            continue

        if ch == '"':
            in_str = True
            cur.append(ch)
            continue

        if ch == ";":
            s = "".join(cur).strip()
            if s:
                out.append(s)
            cur = []
        else:
            cur.append(ch)

    tail = "".join(cur).strip()
    if tail:
        out.append(tail)
    return [s for s in (x.strip() for x in out) if s]


def _parse_stmt(s: str) -> Stmt:
    # say "text"  -> io_println("text")
    m = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\s+(.+)\s*", s)
    if m and m.group(1) in SAY_WORDS:
        return Call(name="io_println", args=[m.group(2).strip()])

    m = re.fullmatch(rf"({'|'.join(map(re.escape, KW_RETURN))})\s+([0-9]+)\s*", s)
    if m:
        return Return(int(m.group(2)))

    m = re.fullmatch(rf"({'|'.join(map(re.escape, KW_LET))})\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.+)", s)
    if m:
        name = m.group(2)
        call = _parse_call(m.group(3).strip())
        return Let(name=name, expr=call)

    return _parse_call(s)


def _parse_call(s: str) -> Call:
    m = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\s*\((.*)\)\s*", s)
    if not m:
        raise RagParseError(f"Unsupported statement: {s!r}")
    name = m.group(1)
    args = _split_args(m.group(2).strip())
    return Call(name=name, args=args)


def _split_args(arg_src: str) -> List[str]:
    if not arg_src:
        return []
    args: List[str] = []
    cur: List[str] = []
    in_str = False
    esc = False
    for ch in arg_src:
        if in_str:
            cur.append(ch)
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            continue

        if ch == '"':
            in_str = True
            cur.append(ch)
            continue

        if ch == ",":
            a = "".join(cur).strip()
            if a:
                args.append(a)
            cur = []
        else:
            cur.append(ch)

    tail = "".join(cur).strip()
    if tail:
        args.append(tail)
    return args


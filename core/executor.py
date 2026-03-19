from __future__ import annotations

import ast
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List

from core.parser import Call, Let, Return, Stmt
from core.keywords import FUNCTION_ALIASES
from libs._45m import builtins as m45m


class RagRuntimeError(Exception):
    pass


def execute(stmts: List[Stmt], project_root: Path) -> int:
    env: Dict[str, Any] = {}

    for st in stmts:
        if isinstance(st, Return):
            return st.value
        if isinstance(st, Let):
            env[st.name] = _eval_call(st.expr, env, project_root)
            continue
        if isinstance(st, Call):
            _eval_call(st, env, project_root)
            continue
        raise RagRuntimeError(f"Unknown statement type: {type(st)}")

    return 0


def _eval_call(call: Call, env: Dict[str, Any], project_root: Path) -> Any:
    name = FUNCTION_ALIASES.get(call.name, call.name)
    fn = m45m.get_function(name)
    if fn is None:
        raise RagRuntimeError(f"Unknown function '{call.name}'")
    args = [_eval_atom(a, env) for a in call.args]
    return fn(project_root, *args)


def _eval_atom(src: str, env: Dict[str, Any]) -> Any:
    src = src.strip()
    if not src:
        return None
    if src in env:
        return env[src]
    if src.isdigit():
        return int(src)
    # string literal
    if src.startswith('"') and src.endswith('"'):
        try:
            return ast.literal_eval(src)
        except Exception:
            return src[1:-1]
    raise RagRuntimeError(f"Unsupported expression: {src!r}")


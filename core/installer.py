from __future__ import annotations

import shutil
from pathlib import Path

from core.prompt import allow_changes_prompt


class InstallError(Exception):
    pass


def install_package(rag_file: Path, packages_dir: Path) -> Path:
    if rag_file.suffix.lower() not in {".i", ".rag"}:
        raise InstallError("Only .i or .rag files can be installed")
    if not rag_file.exists():
        raise InstallError("File not found")

    if not allow_changes_prompt(f"install {rag_file.name}"):
        raise InstallError("User denied permission")

    packages_dir.mkdir(parents=True, exist_ok=True)
    dest = packages_dir / rag_file.name
    shutil.copy2(rag_file, dest)
    return dest


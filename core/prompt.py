from __future__ import annotations


def allow_changes_prompt(action: str) -> bool:
    """
    Secure yes/no prompt (simulates OS permission request).
    """
    while True:
        ans = input(f'Do you want to allow this app to make changes to your device? ({action}) [y/n]: ').strip().lower()
        if ans in {"y", "yes"}:
            return True
        if ans in {"n", "no"}:
            return False


import re
from typing import List, Optional
import psutil


def is_profile_active(names: List[str], pattern: str, exclude: Optional[str]) -> bool:
    for proc in psutil.process_iter():
        if proc.name() in names:
            try:
                cmdline = " ".join(proc.cmdline())
            except (psutil.ZombieProcess, psutil.AccessDenied):
                # Skip zombie processes
                continue

            # Exclude pattern
            if exclude and re.search(exclude, cmdline):
                continue

            if re.search(pattern, cmdline):
                return True
    return False


if __name__ == "__main__":
    print(is_profile_active(["chrome"], r"--user-data-dir.*", r"--type=renderer"))

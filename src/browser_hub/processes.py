import re
from typing import List, Optional
import psutil


def is_profile_active(names: List[str], pattern: str, exclude: Optional[str]) -> bool:
    for proc in psutil.process_iter():
        if proc.name() in names:
            cmdline = proc.cmdline()

            # Exclude pattern
            if exclude:
                for cmd in cmdline:
                    if re.search(exclude, cmd):
                        continue

            for cmd in cmdline:
                if re.search(pattern, cmd):
                    return True
    return False


if __name__ == "__main__":
    print(is_profile_active(["chrome"], r"--user-data-dir.*", r"--type=renderer"))

#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

python3 <<'PY'
from pathlib import Path

lib_path = Path("src/lib.rs")
if not lib_path.exists():
    raise SystemExit("src/lib.rs not found")

text = lib_path.read_text()
line = "pub mod import_contract;"

if line not in text:
    if not text.endswith("\n"):
        text += "\n"
    text += line + "\n"
    lib_path.write_text(text)
    print("added pub mod import_contract; to src/lib.rs")
else:
    print("src/lib.rs already exports import_contract")
PY

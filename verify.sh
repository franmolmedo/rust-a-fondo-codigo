#!/usr/bin/env sh
set -eu

repository_root=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
python3 "$repository_root/tools/verify.py"

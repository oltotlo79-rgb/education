#!/usr/bin/env bash
dir="$1"; shift
cd "$dir" || exit 1
cargo "$@" 2>&1 | PYTHONIOENCODING=utf-8 python "$(dirname "$0")/mask.py"

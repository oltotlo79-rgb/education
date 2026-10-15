#!/usr/bin/env bash
# usage: cap.sh <outname> <dir> <cargo args...>
name="$1"; shift
dir="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
cd "$dir" || exit 1
cargo "$@" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask.py" > "$here/out/$name.txt"
cat "$here/out/$name.txt"

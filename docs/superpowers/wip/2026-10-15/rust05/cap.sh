#!/usr/bin/env bash
# usage: cap.sh <outname> <cargo args...>   (runs in async_lab)
name="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
cd "$here/async_lab" || exit 1
cargo "$@" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask.py" > "$here/out/$name.txt"
cat "$here/out/$name.txt"

#!/usr/bin/env bash
# usage: cap.sh <outname> <dir> <command...>   (runs command in dir, masks paths, saves out/<outname>.txt)
name="$1"; shift
dir="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
cd "$dir" || exit 1
"$@" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask09.py" > "$here/out/$name.txt"
cat "$here/out/$name.txt"

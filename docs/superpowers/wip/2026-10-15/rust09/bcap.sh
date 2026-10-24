#!/usr/bin/env bash
# usage: bcap.sh <outname> <command...>
# book/rustnote で command を実行し、パスを伏せて out2/<outname>.txt に保存する
here="$(cd "$(dirname "$0")" && pwd)"
name="$1"; shift
cd "$here/book/rustnote" || exit 1
export CARGO_TARGET_DIR="$here/target"
"$@" > "$here/out2/$name.raw" 2>&1
code=$?
PYTHONIOENCODING=utf-8 python "$here/bmask.py" < "$here/out2/$name.raw" > "$here/out2/$name.txt"
echo "exit=$code" >> "$here/out2/$name.txt"
cat "$here/out2/$name.txt"

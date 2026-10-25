#!/usr/bin/env bash
# usage: cap.sh <outname> <command...>
# rust13/rustnote で command を実行し、パスを伏せて out/<outname>.txt に保存する
here="$(cd "$(dirname "$0")" && pwd)"
name="$1"; shift
cd "$here/${WORKDIR:-rustnote}" || exit 1
export CARGO_TARGET_DIR="$here/target"
"$@" > "$here/out/$name.raw" 2>&1
code=$?
PYTHONIOENCODING=utf-8 python "$here/mask13.py" < "$here/out/$name.raw" | tr -d '\r' > "$here/out/$name.txt"
echo "exit=$code" >> "$here/out/$name.txt"
cat "$here/out/$name.txt"

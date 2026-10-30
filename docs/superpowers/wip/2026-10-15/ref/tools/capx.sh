#!/usr/bin/env bash
# usage: VOL=rust08 capx.sh <outname> <command...>
# $VOL/${WORKDIR:-rustnote} で command を実行し、パスを伏せて $VOL/out/<outname>.txt に保存する
scratch="$(cd "$(dirname "$0")" && pwd)"
vol="$scratch/$VOL"
name="$1"; shift
cd "$vol/${WORKDIR:-rustnote}" || exit 1
export CARGO_TARGET_DIR="$scratch/target-shared"
mkdir -p "$vol/out"
"$@" > "$vol/out/$name.raw" 2>&1
code=$?
PYTHONIOENCODING=utf-8 python "$scratch/maskx.py" < "$vol/out/$name.raw" | tr -d '\r' > "$vol/out/$name.txt"
echo "exit=$code" >> "$vol/out/$name.txt"
cat "$vol/out/$name.txt"

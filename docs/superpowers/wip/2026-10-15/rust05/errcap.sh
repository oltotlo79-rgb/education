#!/usr/bin/env bash
# usage: errcap.sh <name> [run]   errs/<name>.rs をsrc/bin に置いてビルド（run なら実行）し、出力を out/<name>.txt に保存して消す
name="$1"; mode="${2:-build}"
here="$(cd "$(dirname "$0")" && pwd)"
cp "$here/errs/$name.rs" "$here/async_lab/src/bin/$name.rs"
cd "$here/async_lab" || exit 1
touch "src/bin/$name.rs"
if [ "$mode" = run ]; then
  cargo run --bin "$name" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask.py" > "$here/out/$name.txt"
else
  cargo build --bin "$name" 2>&1 | PYTHONIOENCODING=utf-8 python "$here/mask.py" > "$here/out/$name.txt"
fi
rm "src/bin/$name.rs"
cat "$here/out/$name.txt"

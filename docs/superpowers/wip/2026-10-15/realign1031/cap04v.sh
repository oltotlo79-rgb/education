#!/usr/bin/env bash
# 第4巻の、Rust の版が表示に出る出力を、1.99.1・rust-version "1.99" で取り直す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust04v"
WIP=/c/Users/oltot/Documents/git-projects/def/docs/superpowers/wip/2026-10-15/rust04
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
O="$H/out"; mkdir -p "$O"
export PYTHONIOENCODING=utf-8
mask() { python "$S/maskx.py" | tr -d '\r'; }
tc() { printf '[toolchain]\nchannel = "1.99.1"\n' > "$1/rust-toolchain.toml"; }
cd "$H" || exit 1
rm -rf w; mkdir w; cd w

# ---- 1. design_lab の cargo add（rust-version を足す前）
cargo new -q design_lab && tc design_lab
( cd design_lab && cargo add thiserror anyhow serde_json && cargo add serde --features derive ) 2>&1 | mask > "$O/v04_design_add.txt"

# ---- 2. workspace（rust-version = "1.99"）
cp -r "$WIP/ex06/ws_lab" ws_lab && rm -rf ws_lab/Cargo.lock ws_lab/target && tc ws_lab
python - ws_lab/Cargo.toml <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
assert s.count('rust-version = "1.98"') == 1
open(p, "w", encoding="utf-8", newline="\n").write(s.replace('rust-version = "1.98"', 'rust-version = "1.99"'))
PY
( cd ws_lab && cargo run -p app ) 2>&1 | mask > "$O/v04_ws_run.txt"

# ---- 3. 最小構成の cargo add
cargo new -q minimal && tc minimal
( cd minimal && cargo add serde_json --no-default-features --features alloc ) 2>&1 | mask > "$O/v04_minimal_add.txt"

# ---- 4. rust-version より古い Rust
cp -r "$WIP/feat_lab" feat_lab && rm -rf feat_lab/target && tc feat_lab
python - feat_lab/Cargo.toml <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
assert s.count('rust-version = "1.98"') == 1
open(p, "w", encoding="utf-8", newline="\n").write(s.replace('rust-version = "1.98"', 'rust-version = "1.99"'))
PY
( cd feat_lab && cargo +1.90.0 build ) 2>&1 | mask > "$O/v04_old_rust.txt"
echo done

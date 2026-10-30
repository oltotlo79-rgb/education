#!/usr/bin/env bash
# 第10巻 UNIT 10 の出力を取る（総合の試験と、わざと1か所外したコード）。
# ブラウザでの確かめ（狭い画面・キーボード）は、claude-in-chrome で行い、結果を out/u10_br_*.txt に保存した。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust10
export CARGO_TARGET_DIR="$S/target10"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust10/errs"
cd "$S/rust10/rustnote" || exit 1
restore() { cp "$REAL/src/$1" "src/$1"; }
check() { diff -rq src "$REAL/src" > /dev/null && echo "src は教材と同じ" || { echo "src が教材と違う"; exit 1; }; }

check
cargo build -q --tests || exit 1
$CAP u10_journey cargo test --test u10_journey -- --nocapture > /dev/null

# version の条件を外す（UNIT 04 のエラー1）
$E src/repository/notes.rs "$X/e04_no_version.py"
$CAP u10_err_version cargo test --test u10_journey -- --nocapture > /dev/null
restore repository/notes.rs

# 持ち主の条件を外す（UNIT 03 のエラー1）
$E src/repository/notes.rs "$X/e03_no_owner.py"
$CAP u10_err_owner cargo test --test u10_journey -- --nocapture > /dev/null
restore repository/notes.rs

check
cargo build -q --tests

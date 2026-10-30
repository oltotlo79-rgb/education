#!/usr/bin/env bash
# 第10巻 UNIT 05 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u05_toggle cargo test --test u05_toggle -- --nocapture > /dev/null
$CAP u05_race cargo test --test u05_race -- --nocapture > /dev/null
$CAP u05_q cargo test --test u05_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: 読んでから書く
$E src/repository/notes.rs "$X/e05_read_then_write.py"
$CAP u05_err_race cargo test --test u05_race -- --nocapture > /dev/null
restore repository/notes.rs

# エラー2: done + 1
$E src/repository/notes.rs "$X/e05_plus_one.py"
$CAP u05_err_check cargo test --test u05_toggle -- --nocapture > /dev/null
restore repository/notes.rs

# エラー3: !done（SQL に無い書き方）
$E src/repository/notes.rs "$X/e05_bang.py"
$CAP u05_err_bang cargo test --test u05_toggle -- --nocapture > /dev/null
restore repository/notes.rs

check
cargo build -q --tests

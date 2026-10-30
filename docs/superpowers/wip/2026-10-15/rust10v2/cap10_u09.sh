#!/usr/bin/env bash
# 第10巻 UNIT 09 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u09_flash cargo test --test u09_flash -- --nocapture > /dev/null
$CAP u09_errors cargo test --test u09_errors -- --nocapture > /dev/null
$CAP u09_late cargo test --test u09_late -- --nocapture > /dev/null
$CAP u09_q cargo test --test u09_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: 取り出したお知らせを消さない
$E src/flash.rs "$X/e09_no_remove.py"
$CAP u09_err_remove cargo test --test u09_flash -- --nocapture > /dev/null
restore flash.rs

# エラー2: 500 の画面に原因を出す
$E src/views.rs "$X/e09_leak.py"
$CAP u09_err_leak cargo test --test u09_errors -- --nocapture > /dev/null
restore views.rs

# エラー3: 詳細の画面でお知らせを取り出さない
$E src/routes/notes/id.rs "$X/e09_late.py"
$CAP u09_err_late cargo test --test u09_late -- --nocapture > /dev/null
restore routes/notes/id.rs

check
cargo build -q --tests

#!/usr/bin/env bash
# 第10巻 UNIT 08 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u08_pages cargo test --test u08_pages -- --nocapture > /dev/null
$CAP u08_shift cargo test --test u08_shift -- --nocapture --test-threads=1 > /dev/null
$CAP u08_deep cargo test --test u08_deep -- --nocapture > /dev/null
$CAP u08_q cargo test --test u08_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: OFFSET の計算で 1 を引き忘れる
$E src/domain.rs "$X/e08_offset.py"
$CAP u08_err_offset cargo test --test u08_pages -- --nocapture > /dev/null
$CAP u08_err_offset_lib cargo test --lib > /dev/null
restore domain.rs

# エラー2: ページの数を切り捨てで数える
$E src/domain.rs "$X/e08_pages.py"
$CAP u08_err_pages cargo test --test u08_pages -- --nocapture > /dev/null
$CAP u08_err_pages_lib cargo test --lib > /dev/null
restore domain.rs

# エラー3: ページ送りのリンクで q を引き継がない
$E src/views.rs "$X/e08_keep_q.py"
$CAP u08_err_q cargo test --test u08_pages -- --nocapture > /dev/null
restore views.rs

check
cargo build -q --tests

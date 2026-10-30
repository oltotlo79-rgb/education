#!/usr/bin/env bash
# 第10巻 UNIT 07 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u07_search cargo test --test u07_search -- --nocapture > /dev/null
$CAP u07_like cargo test --test u07_like -- --nocapture > /dev/null
$CAP u07_inject cargo test --test u07_inject -- --nocapture > /dev/null
$CAP u07_q cargo test --test u07_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: escape_like を通さない
$E src/repository/notes.rs "$X/e07_no_escape.py"
$CAP u07_err_escape cargo test --test u07_search -- --nocapture > /dev/null
restore repository/notes.rs

# エラー2: ESCAPE '\' を外す
$E src/repository/notes.rs "$X/e07_no_escape_clause.py"
$CAP u07_err_clause cargo test --test u07_search -- --nocapture > /dev/null
restore repository/notes.rs

# エラー3: キーワードを SQL の文字列に直接入れる
$E src/repository/notes.rs "$X/e07_injection.py"
$CAP u07_err_inject cargo test --test u07_inject -- --nocapture > /dev/null
restore repository/notes.rs

check
cargo build -q --tests

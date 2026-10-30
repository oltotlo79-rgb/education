#!/usr/bin/env bash
# 第10巻 UNIT 03 の出力を取る（正しいコードと、わざと1か所外したコード）。
# 使い方: bash cap10_u03.sh
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
$CAP u03_detail cargo test --test u03_detail -- --nocapture --test-threads=1 > /dev/null
$CAP u03_page cargo test --test u03_page -- --nocapture > /dev/null
$CAP u03_q cargo test --test u03_q -- --nocapture --test-threads=1 > /dev/null
$CAP u03_count cargo test --test u03_count -- --nocapture > /dev/null

# エラー1: 持ち主の条件を外す
$E src/repository/notes.rs "$X/e03_no_owner.py"
$CAP u03_err_owner cargo test --test u03_detail -- --nocapture --test-threads=1 > /dev/null
restore repository/notes.rs

# エラー2: 他人のノートを 403 にする
$E src/error.rs "$X/e03_forbidden_error.py"
$E src/routes/notes/id.rs "$X/e03_forbidden_find.py"
$CAP u03_err_403 cargo test --test u03_count -- --nocapture > /dev/null
restore error.rs
restore routes/notes/id.rs

# エラー3: href! に NoteId をそのまま渡す
$E src/routes/notes/id.rs "$X/e03_href.py"
$CAP u03_err_href cargo build > /dev/null
restore routes/notes/id.rs

check
cargo build -q --tests

#!/usr/bin/env bash
# 第10巻 UNIT 04 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u04_edit cargo test --test u04_edit -- --nocapture > /dev/null
$CAP u04_conflict cargo test --test u04_conflict -- --nocapture > /dev/null
$CAP u04_q cargo test --test u04_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: version の条件を外す
$E src/repository/notes.rs "$X/e04_no_version.py"
$CAP u04_err_lost cargo test --test u04_conflict -- --nocapture > /dev/null
restore repository/notes.rs

# エラー2: フォームから hidden の version を外す
$E src/views.rs "$X/e04_no_hidden.py"
$CAP u04_err_hidden cargo test --test u04_edit -- --nocapture > /dev/null
restore views.rs

# エラー3: Option<i64> の場所に i64 を入れる
$E src/routes/notes/id/edit.rs "$X/e04_option.py"
$CAP u04_err_option cargo build > /dev/null
restore routes/notes/id/edit.rs

check
cargo build -q --tests

#!/usr/bin/env bash
# 第10巻 UNIT 06 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u06_delete cargo test --test u06_delete -- --nocapture > /dev/null
$CAP u06_guard cargo test --test u06_guard -- --nocapture > /dev/null
$CAP u06_q cargo test --test u06_q -- --nocapture > /dev/null

# エラー1: 削除を GET のリンクにする
$E src/routes/notes/id/delete.rs "$X/e06_get_route.py"
$E src/routes/notes/id.rs "$X/e06_get_link.py"
$CAP u06_err_get cargo test --test u06_guard -- --nocapture > /dev/null
restore routes/notes/id/delete.rs
restore routes/notes/id.rs

# エラー2: 消した行の数を見ない
$E src/repository/notes.rs "$X/e06_rows.py"
$CAP u06_err_rows cargo test --test u06_guard -- --nocapture > /dev/null
restore repository/notes.rs

# エラー3: #[page(DELETE)]
$E src/routes/notes/id/delete.rs "$X/e06_method.py"
$CAP u06_err_method cargo test --test u06_delete -- --nocapture > /dev/null
restore routes/notes/id/delete.rs

check
cargo build -q --tests

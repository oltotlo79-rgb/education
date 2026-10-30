#!/usr/bin/env bash
# 第8巻 UNIT 07 の出力を取る（正しいコードと、わざと1か所外したコード）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust08
export CARGO_TARGET_DIR="$S/target08"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust08/errs"
cd "$S/rust08/rustnote" || exit 1
restore() { cp "$REAL/$1" "$1"; }
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }

check
cargo build -q --tests || exit 1
$CAP u07_matrix cargo test --test u07_matrix -- --nocapture > /dev/null
$CAP u07_ids cargo test --test u07_ids -- --nocapture > /dev/null
$CAP u07_search cargo test --test u07_search -- --nocapture > /dev/null

# エラー1: 完了の切り替えの持ち主の条件を外す
$E src/repository/notes.rs "$X/e07_toggle.py"
$CAP u07_err_toggle cargo test --test u07_matrix -- --nocapture > /dev/null
restore src/repository/notes.rs

# エラー2: 検索の条件のかっこを外す
$E src/repository/notes.rs "$X/e07_precedence.py"
$CAP u07_err_precedence cargo test --test u07_search -- --nocapture > /dev/null
restore src/repository/notes.rs

# エラー3: 持ち主を渡し忘れる
$E src/routes/notes/id/toggle.rs "$X/e07_arity.py"
$CAP u07_err_arity cargo build > /dev/null
restore src/routes/notes/id/toggle.rs

check
cargo build -q --tests

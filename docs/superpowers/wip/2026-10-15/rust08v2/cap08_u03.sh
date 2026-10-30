#!/usr/bin/env bash
# 第8巻 UNIT 03 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u03_signup cargo test --test u03_signup -- --nocapture --test-threads=1 > /dev/null
$CAP u03_race cargo test --test u03_race -- --nocapture > /dev/null
$CAP u03_q cargo test --test u03_q -- --nocapture --test-threads=1 > /dev/null

# エラー1: UNIQUE を外して、先に探してから登録する
$E migrations/0001_users.sql "$X/e03_unique_sql.py"
$E src/routes/signup.rs "$X/e03_unique_rs.py"
$CAP u03_err_unique cargo test --test u03_race -- --nocapture > /dev/null
restore migrations/0001_users.sql
restore src/routes/signup.rs

# エラー2: 小文字にそろえない
$E src/domain.rs "$X/e03_lower.py"
$CAP u03_err_lower cargo test --test u03_signup -- --nocapture --test-threads=1 > /dev/null
restore src/domain.rs

# エラー3: 検査していない String を渡す
$E src/routes/signup.rs "$X/e03_type.py"
$CAP u03_err_type cargo build > /dev/null
restore src/routes/signup.rs

check
cargo build -q --tests

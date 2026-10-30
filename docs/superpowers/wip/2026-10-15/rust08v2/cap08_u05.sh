#!/usr/bin/env bash
# 第8巻 UNIT 05 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
$CAP u05_life cargo test --test u05_life -- --nocapture > /dev/null
$CAP u05_server cargo test --test u05_server -- --nocapture --test-threads=1 > /dev/null
$CAP u05_stolen cargo test --test u05_stolen -- --nocapture > /dev/null
$CAP u05_q cargo test --test u05_q -- --nocapture > /dev/null

# エラー1: ログアウトで DB の記録を消さない
$E src/auth.rs "$X/e05_logout.py"
$CAP u05_err_logout cargo test --test u05_stolen -- --nocapture > /dev/null
restore src/auth.rs

# エラー2: 期限を見ない
$E src/repository/sessions.rs "$X/e05_expiry.py"
$CAP u05_err_expiry cargo test --test u05_server expired -- --nocapture > /dev/null
restore src/repository/sessions.rs

# エラー3: 生のトークンを保存しようとする
$E src/auth.rs "$X/e05_token.py"
$CAP u05_err_token cargo build > /dev/null
restore src/auth.rs

check
cargo build -q --tests

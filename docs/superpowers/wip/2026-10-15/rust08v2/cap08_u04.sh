#!/usr/bin/env bash
# 第8巻 UNIT 04 の出力を取る（正しいコードと、わざと1か所外したコード）。
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
cargo build -q --tests --examples || exit 1
$CAP u04_login cargo test --test u04_login -- --nocapture --test-threads=1 > /dev/null
$CAP u04_renew cargo test --test u04_renew -- --nocapture > /dev/null
$CAP u04_timing cargo test --test u04_timing -- --nocapture > /dev/null
$CAP u04_limiter cargo run -q --example u04_limiter > /dev/null
$CAP u04_q cargo test --test u04_q -- --nocapture > /dev/null

# エラー1: 失敗の理由で文を変える
$E src/routes/login.rs "$X/e04_message.py"
$CAP u04_err_message cargo test --test u04_login -- --nocapture --test-threads=1 > /dev/null
restore src/routes/login.rs

# エラー2: 登録が無ければ、照合せずにすぐ返す
$E src/auth.rs "$X/e04_dummy.py"
$CAP u04_err_dummy cargo test --test u04_timing -- --nocapture > /dev/null
restore src/auth.rs

# エラー3: .await を忘れる
$E src/auth.rs "$X/e04_await.py"
$CAP u04_err_await cargo build > /dev/null
restore src/auth.rs

check
cargo build -q --tests

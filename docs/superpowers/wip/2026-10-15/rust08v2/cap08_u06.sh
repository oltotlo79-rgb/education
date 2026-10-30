#!/usr/bin/env bash
# 第8巻 UNIT 06 の出力を取る（正しいコードと、わざと1か所変えたコード）。
# ブラウザでの確かめ（HttpOnly・SameSite・ほかのサイトのフォームとリンク）は claude-in-chrome で行い、out/u06_br_*.txt に保存した。
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
$CAP u06_policy cargo test --test u06_policy -- --nocapture > /dev/null
$CAP u06_trust cargo test --test u06_trust -- --nocapture > /dev/null

# エラー1: OriginPolicy を外す
$E src/routes.rs "$X/e06_disable.py"
$CAP u06_err_disable cargo test --test u06_policy -- --nocapture > /dev/null
restore src/routes.rs

# エラー2: 信じる送り元の最後に / を付ける（と、直した版）
$E src/routes.rs "$X/e06_slash.py"
$CAP u06_err_slash cargo test --test u06_trust -- --nocapture > /dev/null
restore src/routes.rs
$E src/routes.rs "$X/e06_ok.py"
$CAP u06_fix_slash cargo test --test u06_trust -- --nocapture > /dev/null
restore src/routes.rs

# エラー3: SameSite の値を書き間違える
$E src/flash.rs "$X/e06_samesite.py"
$CAP u06_err_samesite cargo build > /dev/null
restore src/flash.rs

check
cargo build -q --tests

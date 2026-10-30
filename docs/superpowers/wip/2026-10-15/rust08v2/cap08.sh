#!/usr/bin/env bash
# 第8巻 UNIT 01〜02 の出力を、今の版（Rust 1.99.1・Topcoat 0.11）で取り直す。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust08
export CARGO_TARGET_DIR="$S/target08"
CAP="bash $S/capx.sh"
cd "$S/rust08/rustnote" || exit 1
diff -rq src "$REAL/src" > /dev/null || { echo "src が教材と違う"; exit 1; }
cargo build -q --all-targets || exit 1
cargo build -q --release --example u02_speed || exit 1

$CAP u01_three cargo test --test u01_three_checks -- --nocapture --test-threads=1 > /dev/null
$CAP u01_session cargo test --test u01_session -- --nocapture > /dev/null
$CAP u01_cross cargo test --test u01_cross_site -- --nocapture > /dev/null
cp "$S/rust08/errs/e01_find_without_owner.rs" examples/e01_find_without_owner.rs
$CAP u01_err_owner cargo build --example e01_find_without_owner > /dev/null
rm examples/e01_find_without_owner.rs
$CAP u01_idor cargo run -q --example u01_idor > /dev/null
$CAP u01_q1 cargo test --test u01_q1 -- --nocapture > /dev/null
$CAP u01_q2 cargo test --test u01_q2 -- --nocapture > /dev/null

$CAP u02_hash cargo run -q --example u02_hash > /dev/null
$CAP u02_speed cargo run -q --release --example u02_speed > /dev/null
$CAP u02_stored cargo test --test u02_stored -- --nocapture --test-threads=1 > /dev/null
cp "$S/rust08/errs/e02_old_api.rs" examples/e02_old_api.rs
$CAP u02_err_old_api cargo build --example e02_old_api > /dev/null
rm examples/e02_old_api.rs
$CAP u02_q2 cargo test --test u02_q2 -- --nocapture > /dev/null
$CAP u02_q3 cargo test --test u02_q3 -- --nocapture > /dev/null
diff -rq src "$REAL/src" && echo "src は教材と同じ"

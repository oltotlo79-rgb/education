#!/usr/bin/env bash
# 第8巻 UNIT 10 の出力を取る（今の RustNote・監査の記録を足した版・わざと間違えた版）。
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
check() { [ ! -e src/audit.rs ] && diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }

check
cargo build -q --tests || exit 1
$CAP u10_guards cargo test --test u10_guards -- --nocapture > /dev/null
$CAP u10_audit_now cargo test --test u10_audit -- --nocapture > /dev/null
$CAP u10_incident cargo test --test u10_incident -- --nocapture > /dev/null

# 監査の記録を足した版
cp "$S/rust08/variants/audit.rs" src/audit.rs
$E src/lib.rs "$X/e10_audit_lib.py"
$E src/routes/login.rs "$X/e10_audit_login.py"
$E src/routes/logout.rs "$X/e10_audit_logout.py"
cargo build -q --tests || exit 1
$CAP u10_audit_added cargo test --test u10_audit -- --nocapture > /dev/null
diff -u --label a/src/routes/login.rs --label b/src/routes/login.rs "$REAL/src/routes/login.rs" src/routes/login.rs > "$S/rust08/out/u10_diff_login.txt"
diff -u --label a/src/routes/logout.rs --label b/src/routes/logout.rs "$REAL/src/routes/logout.rs" src/routes/logout.rs > "$S/rust08/out/u10_diff_logout.txt"

# エラー2: フォームをまるごと記録に書く（E0277）
$E src/routes/login.rs "$X/e10_form_log.py"
$CAP u10_err_debug cargo build > /dev/null
# エラー3: Debug を足してコンパイルを通す（パスワードが記録に出る）
$E src/routes/login.rs "$X/e10_form_debug.py"
$CAP u10_err_password cargo test --test u10_audit -- --nocapture > /dev/null
restore src/routes/login.rs
restore src/routes/logout.rs
restore src/lib.rs
rm src/audit.rs

# エラー1: ログアウトで DB の記録を消さない（UNIT 05 と同じ間違い）→ 守りのテストが止まる
$E src/auth.rs "$X/e05_logout.py"
$CAP u10_err_logout cargo test --test u10_guards -- --nocapture > /dev/null
restore src/auth.rs

# 問3: 守りのテストから Sec-Fetch-Site を外す
cp tests/u10_guards.rs "$S/rust08/u10_guards.keep"
$E tests/u10_guards.rs "$X/e10_q_header.py"
$CAP u10_q_header cargo test --test u10_guards -- --nocapture > /dev/null
cp "$S/rust08/u10_guards.keep" tests/u10_guards.rs
rm "$S/rust08/u10_guards.keep"

check
cargo build -q --tests

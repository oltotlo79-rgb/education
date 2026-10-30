#!/usr/bin/env bash
# 第11巻 UNIT 10 の出力を取る（CI の手順を手元で・cargo audit・依存の機能を減らす）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export WORKDIR=rustnote_u07
export CARGO_TARGET_DIR="$S/target11b"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
O="$S/rust11/out"
cd "$S/rust11/rustnote_u07" || exit 1
# このコピーを、教材と同じにする（tests は http.rs・repository.rs・common だけ）
for f in Cargo.toml Cargo.lock rust-toolchain.toml rustfmt.toml; do cp "$REAL/$f" "$f"; done
rm -rf src && cp -r "$REAL/src" src
rm -rf tests && mkdir tests && cp -r "$REAL/tests/common" "$REAL/tests/http.rs" "$REAL/tests/repository.rs" tests/
diff -rq src "$REAL/src" > /dev/null && diff -q Cargo.toml "$REAL/Cargo.toml" > /dev/null && diff -q Cargo.lock "$REAL/Cargo.lock" > /dev/null || { echo "教材と違う"; exit 1; }

# ---- 例10-1: CI の3つの手順を、手元で
$CAP u10_ci_fmt cargo fmt --check > /dev/null
$CAP u10_ci_clippy cargo clippy --all-targets --locked -- -D warnings > /dev/null
$CAP u10_ci_test cargo test --locked > /dev/null

# ---- 例10-2: cargo audit
$CAP u10_audit_version cargo audit --version > /dev/null
$CAP u10_audit cargo audit > /dev/null
$CAP u10_tree_webpki cargo tree -i rustls-webpki@0.102.8 > /dev/null
$CAP u10_tree_h2 cargo tree -i h2@0.3.27 --depth 4 > /dev/null
grep -c '^name = ' Cargo.lock > "$O/u10_lock_count_before.txt"

# ---- 例10-3: libsql の使わない機能を外す
$E Cargo.toml "$X/e10_features.py" > /dev/null || exit 1
diff -u --label a/Cargo.toml --label b/Cargo.toml "$REAL/Cargo.toml" Cargo.toml > "$O/u10_diff_features.txt"
$CAP u10_features_build cargo build --tests > /dev/null
$CAP u10_features_test cargo test --lib --test http --test repository > /dev/null
grep -c '^name = ' Cargo.lock > "$O/u10_lock_count_after.txt"
$CAP u10_features_audit cargo audit > /dev/null
cp "$REAL/Cargo.toml" Cargo.toml; cp "$REAL/Cargo.lock" Cargo.lock

# ---- 例10-4: 受け入れると決めた弱点を、理由つきで記録する
mkdir -p .cargo
cat > .cargo/audit.toml <<'TOML'
# cargo audit の設定。直せない弱点を、理由と見直す日を書いて、受け入れる
[advisories]
ignore = [
    # libsql 0.10.0（最新）の Turso への接続の部品。libsql の更新を待つ（2026-11-30 に見直す）
    "RUSTSEC-2026-0258", # h2 0.3.27
    "RUSTSEC-2026-0346", # rustls 0.22.4
    "RUSTSEC-2026-0349", # rustls 0.22.4
    "RUSTSEC-2026-0049", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0098", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0099", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0104", # rustls-webpki 0.102.8
    "RUSTSEC-2026-0382", # rustls-webpki 0.102.8
]
TOML
cp .cargo/audit.toml "$O/u10_audit_toml.txt"
$CAP u10_audit_ignored cargo audit > /dev/null
rm -rf .cargo

# ---- エラー1: Cargo.lock を更新せずに --locked
$E Cargo.toml "$X/e10_new_dep.py" > /dev/null || exit 1
$CAP u10_err_locked cargo build --locked > /dev/null
cp "$REAL/Cargo.toml" Cargo.toml; cp "$REAL/Cargo.lock" Cargo.lock

# ---- エラー2: 使わない変数（-D warnings）
$E src/routes/health.rs "$X/e10_unused.py" > /dev/null || exit 1
$CAP u10_err_build cargo build > /dev/null
$CAP u10_err_clippy cargo clippy --all-targets --locked -- -D warnings > /dev/null
cp "$REAL/src/routes/health.rs" src/routes/health.rs

# ---- エラー3: 字下げを崩した（fmt --check）
$E src/routes/health.rs "$X/e10_format.py" > /dev/null || exit 1
$CAP u10_err_fmt cargo fmt --check > /dev/null
cp "$REAL/src/routes/health.rs" src/routes/health.rs

diff -rq src "$REAL/src" > /dev/null && diff -q Cargo.toml "$REAL/Cargo.toml" > /dev/null && diff -q Cargo.lock "$REAL/Cargo.lock" > /dev/null && echo "教材と同じに戻した"

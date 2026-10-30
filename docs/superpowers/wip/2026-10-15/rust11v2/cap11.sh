#!/usr/bin/env bash
# 第11巻 UNIT 01〜02 の出力を、今の RustNote（Topcoat 0.11・Rust 1.99.1）で取り直す。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
cd "$S/rust11/rustnote" || exit 1
restore() { cp "$REAL/$1" "$1"; }
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
SHARED="$(cygpath -u "$TEMP")/rustnote-shared-test.db"
clear_shared() { rm -f "$SHARED" "$SHARED"-*; }

# 教材の版の設定にそろえる（tests/ の u01・u02 の確かめ用のファイルは残す）
for f in Cargo.toml Cargo.lock rust-toolchain.toml ci.yml Dockerfile fly.toml README.md rustfmt.toml; do cp "$REAL/$f" "$f"; done
cp "$REAL/tests/http.rs" "$REAL/tests/repository.rs" tests/
cp -r "$REAL/tests/common" tests/
check
cargo build -q --tests --examples || exit 1

# ---- UNIT 01
$CAP u01_table cargo test --test u01_title_table -- --nocapture --test-threads=1 > /dev/null
$CAP u01_counts cargo run -q --example u01_counts > /dev/null
for pair in "e01_wrong_row:u01_err_row" "e02_async:u01_err_async" "e03_no_assert:u01_err_noassert"; do
  file=${pair%%:*}; key=${pair##*:}
  cp "$X/$file.rs" "tests/$file.rs"
  $CAP "$key" cargo test --test "$file" > /dev/null
  rm "tests/$file.rs"
done
for q in 1 2 3; do $CAP u01_q$q cargo test --test u01_q$q -- --nocapture > /dev/null; done

# ---- UNIT 02
$CAP u02_contract cargo test --test u02_contract -- --nocapture > /dev/null
$CAP u02_repo cargo test --test repository > /dev/null

# 例2-2: ESCAPE を消す
$E src/repository/notes.rs "$X/e02_no_escape.py"
$CAP u02_mut_escape_unit cargo test --lib like_special > /dev/null
$CAP u02_mut_escape cargo test --test repository > /dev/null
restore src/repository/notes.rs

# エラー1: 削除の持ち主の条件を消す
$E src/repository/notes.rs "$X/e02_no_owner.py"
$CAP u02_mut_delete cargo test --test repository > /dev/null
$CAP u02_owner_delete_broken cargo test --test u02_owner_delete > /dev/null
restore src/repository/notes.rs
$CAP u02_owner_delete cargo test --test u02_owner_delete > /dev/null

# エラー2: 同じ DB ファイルを使い回す
cargo build -q --tests
clear_shared
$CAP u02_shared1 cargo test --test u02_shared > /dev/null
$CAP u02_shared2 cargo test --test u02_shared > /dev/null
# ファイルを消してから 8 回くり返し、失敗の理由を数える
: > "$S/rust11/out/u02_shared_runs.txt"
for i in 1 2 3 4 5 6 7 8; do
  clear_shared
  $CAP u02_shared_run$i cargo test --test u02_shared > /dev/null
  reason=$(grep -o "UniqueViolation\|database is locked\|already exists" "$S/rust11/out/u02_shared_run$i.txt" | head -1)
  passed=$(grep -c "\.\.\. ok" "$S/rust11/out/u02_shared_run$i.txt")
  echo "run$i ok=$passed reason=${reason:-なし}" >> "$S/rust11/out/u02_shared_runs.txt"
done
clear_shared

for q in 1 2 3; do $CAP u02_q$q cargo test --test u02_q$q -- --nocapture > /dev/null; done

check
cat "$S/rust11/out/u02_shared_runs.txt"

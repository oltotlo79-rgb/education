#!/usr/bin/env bash
# 第11巻 UNIT 03 の出力を取る。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
O="$S/rust11/out"
cd "$S/rust11/rustnote" || exit 1
restore() { cp "$REAL/$1" "$1"; }
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check

# scraper を dev-dependency に足す（足したときの表示を取る）
cargo remove -q --dev scraper
$CAP u03_add cargo add --dev scraper@=0.29.0 > /dev/null
cargo build -q --tests || exit 1

$CAP u03_parts cargo test --test u03_parts -- --nocapture > /dev/null
$CAP u03_helpers cargo test --test u03_helpers > /dev/null
$CAP u03_body cargo test --test u03_body -- --nocapture --test-threads=1 > /dev/null

# 例3-2: #[track_caller] の有る・無し
for f in e03_no_track e03_track; do
  cp "$X/$f.rs" tests/
  $CAP "u03_$f" cargo test --test "$f" > /dev/null
  rm "tests/$f.rs"
done

# エラー1: 本文を読んだあとで、状態コードを見る（E0382）
cp "$X/e03_moved.rs" tests/
$CAP u03_err_moved cargo test --test e03_moved > /dev/null
rm tests/e03_moved.rs

# エラー2: 1つ目の Set-Cookie を、セッションだと思い込む（10 回くり返して数える）
cp "$X/e03_first_cookie.rs" tests/
cargo build -q --tests
: > "$O/u03_first_cookie_runs.txt"
for i in 1 2 3 4 5 6 7 8 9 10; do
  $CAP u03_first_cookie_run cargo test --test e03_first_cookie > /dev/null
  if grep -q "test result: ok" "$O/u03_first_cookie_run.txt"; then
    echo "$i 回目: ok" >> "$O/u03_first_cookie_runs.txt"
    cp "$O/u03_first_cookie_run.txt" "$O/u03_first_cookie_ok.txt"
  else
    echo "$i 回目: FAILED" >> "$O/u03_first_cookie_runs.txt"
    cp "$O/u03_first_cookie_run.txt" "$O/u03_first_cookie_failed.txt"
  fi
done
rm tests/e03_first_cookie.rs "$O"/u03_first_cookie_run.*

# エラー3: 検索を「前に一致」にしてしまう → contains は通り、セレクターは止まる
$E src/repository/notes.rs "$X/e03_prefix.py"
$CAP u03_err_prefix cargo test --test u03_body -- --nocapture --test-threads=1 > /dev/null
$CAP u03_err_prefix_http cargo test --test http > /dev/null
$CAP u03_err_prefix_repo cargo test --test repository > /dev/null
restore src/repository/notes.rs

for q in 1 2 3; do $CAP u03_q$q cargo test --test u03_q$q -- --nocapture > /dev/null; done

check
cargo build -q --tests
cat "$O/u03_first_cookie_runs.txt"

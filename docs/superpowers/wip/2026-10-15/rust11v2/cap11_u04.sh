#!/usr/bin/env bash
# 第11巻 UNIT 04 の出力を取る（Windows 側。fuzz は cap11_fuzz.sh を WSL で動かす）。
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
rm -f tests/*.proptest-regressions

# proptest を dev-dependency に足す（足したときの表示を取る）
cargo remove -q --dev proptest
$CAP u04_add cargo add --dev proptest@=1.12.0 > /dev/null
cargo build -q --tests || exit 1

# 例4-1: NoteTitle の決まり
$CAP u04_title_prop cargo test --test u04_title_prop -- --nocapture --test-threads=1 > /dev/null

# 例4-2: 縮める（1回目と、保存された種で動かす2回目）
$CAP u04_shrink1 cargo test --test u04_shrink -- --test-threads=1 > /dev/null
cp tests/u04_shrink.proptest-regressions "$O/u04_shrink_regressions.txt"
$CAP u04_shrink2 cargo test --test u04_shrink -- --test-threads=1 > /dev/null

# エラー3 の直し方: 空白に寄せた作り方で3回
for i in 1 2 3; do
  rm -f tests/u04_spaces.proptest-regressions
  $CAP "u04_spaces$i" cargo test --test u04_spaces > /dev/null
done
rm -f tests/u04_spaces.proptest-regressions

# NoteQuery の offset（proptest では見つからない）
$CAP u04_query_prop cargo test --test u04_query_prop > /dev/null

# 例4-3: でたらめな HTTP の入力（NUL の題で 500）
# 300 回の試しを 10 回くり返し、NUL の題（500）が見つかった回を数える
: > "$O/u04_http_prop_runs.txt"
rm -f "$O/u04_http_prop.txt" "$O/u04_http_prop_missed.txt"
for i in 1 2 3 4 5 6 7 8 9 10; do
  $CAP u04_http_prop_run cargo test --test u04_http_prop -- --nocapture > /dev/null
  if grep -q "test result: ok" "$O/u04_http_prop_run.txt"; then
    echo "$i 回目: 500 は見つからなかった" >> "$O/u04_http_prop_runs.txt"
    [ -e "$O/u04_http_prop_missed.txt" ] || cp "$O/u04_http_prop_run.txt" "$O/u04_http_prop_missed.txt"
  else
    echo "$i 回目: $(grep -o 'Reason("[^"]*' "$O/u04_http_prop_run.txt" | sed 's/Reason("//; s/ at tests.*//')" >> "$O/u04_http_prop_runs.txt"
    [ -e "$O/u04_http_prop.txt" ] || cp "$O/u04_http_prop_run.txt" "$O/u04_http_prop.txt"
  fi
done
rm -f "$O"/u04_http_prop_run.*
$CAP u04_nul cargo test --test u04_nul -- --nocapture --test-threads=1 > /dev/null

# 直した版（題の制御文字を断る）で、もう一度
$E src/domain.rs "$X/e04_no_control.py"
diff -u --label a/src/domain.rs --label b/src/domain.rs "$REAL/src/domain.rs" src/domain.rs > "$O/u04_diff_domain.txt"
rm -f tests/u04_http_prop.proptest-regressions
$CAP u04_http_prop_fixed cargo test --test u04_http_prop -- --nocapture > /dev/null
$CAP u04_nul_fixed cargo test --test u04_nul -- --nocapture --test-threads=1 > /dev/null
$CAP u04_fixed_lib cargo test --lib > /dev/null
$CAP u04_fixed_http cargo test --test http > /dev/null
restore src/domain.rs
rm -f tests/*.proptest-regressions

# 問1: Email::parse
$CAP u04_q1 cargo test --test u04_q1 -- --nocapture --test-threads=1 > /dev/null
rm -f tests/*.proptest-regressions

# エラー1: 正規表現の \ を1つだけ書いた
cp "$X/e04_escape.rs" tests/
$CAP u04_err_escape cargo test --test e04_escape > /dev/null
rm tests/e04_escape.rs

# エラー2: run に渡す関数の中で、外の変数を書き換えた
cp "$X/e04_fnmut.rs" tests/
$CAP u04_err_fnmut cargo test --test e04_fnmut > /dev/null
rm tests/e04_fnmut.rs

check
cargo build -q --tests

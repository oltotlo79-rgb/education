#!/usr/bin/env bash
# 第11巻 UNIT 06 の出力を取る（守りの回帰テストと、守りを1つずつ外した版）。
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
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
cargo build -q --tests || exit 1

$CAP u06_attacks cargo test --test u06_attacks > /dev/null
$CAP u06_ignored cargo test --test u06_attacks -- --ignored > /dev/null
$CAP u06_include cargo test --test u06_attacks -- --include-ignored > /dev/null
$CAP u06_q1 cargo test --test u06_q1 -- --nocapture > /dev/null

# 守りを1つずつ外して、u06_attacks と、RustNote の tests/http.rs のどれが止まるかを見る
: > "$O/u06_matrix.txt"
for name in toggle delete search origin logout; do
  read -r file table < <(python "$X/e06_variants.py" "$name" | tr -d '\r')
  $E "$file" "$table" > /dev/null || { echo "書き換えに失敗: $name"; exit 1; }
  $CAP "u06_kill_${name}_attacks" cargo test --test u06_attacks > /dev/null
  $CAP "u06_kill_${name}_http" cargo test --test http > /dev/null
  failed_attacks=$(grep -o "^test [a-z_]* \.\.\. FAILED" "$O/u06_kill_${name}_attacks.txt" | sed 's/^test //; s/ \.\.\. FAILED//' | tr '\n' ' ')
  failed_http=$(grep -o "^test [a-z_0-9]* \.\.\. FAILED" "$O/u06_kill_${name}_http.txt" | sed 's/^test //; s/ \.\.\. FAILED//' | tr '\n' ' ')
  echo "$name | u06_attacks: ${failed_attacks:-（全部通った）}| http.rs: ${failed_http:-（全部通った）}" >> "$O/u06_matrix.txt"
  cp "$REAL/$file" "$file"
done
check
cargo build -q --tests
cat "$O/u06_matrix.txt"

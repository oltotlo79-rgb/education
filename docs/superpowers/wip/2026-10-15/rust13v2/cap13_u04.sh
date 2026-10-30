#!/usr/bin/env bash
# 第13巻 UNIT 04 の出力を取る（ドメインの型とエラーの分類）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
cargo build -q --examples || exit 1

# ---- 例4-3: いろいろな入力
$CAP u04_types cargo run -q --example u04_types > /dev/null
# ---- 2つのファイルのユニットテスト
$CAP u04_lib_tests cargo test --lib -- domain:: error:: > /dev/null

# ---- 問1・問2
$CAP u04_q1 cargo run -q --example u04_q1 > /dev/null
$CAP u04_q2 cargo run -q --example u04_q2 > /dev/null
$CAP u04_q2_release cargo run -q --release --example u04_q2 > /dev/null

# ---- エラー1〜3
for e in e04_private e04_mixid e04_question; do
  cp "$H/errs_u04/$e.rs" examples/
  $CAP "u04_err_${e#e04_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done

# ---- 問3: AppError に種類を1つ足す
python - "$H/rustnote/src/error.rs" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = "    Conflict,\n"
assert s.count(old) == 1
new = old + "    #[error(\"ノートの数が上限に達しました\")]\n    TooManyNotes,\n"
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
$CAP u04_q3 cargo build > /dev/null
cp "$REAL/src/error.rs" src/error.rs

check

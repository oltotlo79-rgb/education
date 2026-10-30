#!/usr/bin/env bash
# 第13巻 UNIT 03 の出力を取る（設定と起動の順番）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
EXE="$H/target/debug/rustnote.exe"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
cargo build -q --examples && cargo build -q && topcoat asset bundle > /dev/null || exit 1

# ---- 例3-2: 環境変数の組み合わせ
$CAP u03_config cargo run -q --example u03_config > /dev/null
# ---- 例3-1 の確かめ: config のテスト
$CAP u03_config_tests cargo test --lib config > /dev/null

# ---- 例3-3: 本物の実行ファイルで、起動のどの段で止まるか
RUN="$H/u03run"; rm -rf "$RUN"; mkdir -p "$RUN"; cd "$RUN" || exit 1
mask() { PYTHONIOENCODING=utf-8 python "$H/mask13.py" | tr -d '\r'; }
files() { ls -1 "$RUN" | grep -v '^pid$' | tr '\n' ' '; }
run_case() {  # 名前・環境変数… （止まるまで待つ。サーバーが起動したら止める）
  local name=$1; shift
  ( env "$@" "$EXE" > "$O/$name.stdout" 2> "$O/$name.raw" & echo $! > "$RUN/pid" )
  for i in $(seq 1 20); do
    sleep 0.5
    if grep -q "RustNote を起動しました" "$O/$name.raw" 2>/dev/null; then
      curl.exe -s -i http://127.0.0.1:3014/health | head -1 | tr -d '\r' > "$O/${name}_health.txt"
      taskkill //F //IM rustnote.exe > /dev/null 2>&1; break
    fi
    kill -0 "$(cat "$RUN/pid")" 2> /dev/null || break
  done
  sleep 1
  mask < "$O/$name.raw" > "$O/$name.txt"
  echo "（このあとのフォルダの中: $(files)）" >> "$O/$name.txt"
  rm -f "$O/$name.raw" "$O/$name.stdout" "$RUN/pid"
}
run_case u03_start_ok PORT=3014 DATABASE_URL=ok.db
run_case u03_start_token PORT=3014 DATABASE_URL=libsql://rustnote-demo.turso.io
run_case u03_start_port PORT=abc DATABASE_URL=port.db
# 束（assets/）の無いフォルダの実行ファイル
mkdir -p "$H/u03exe" && cp "$EXE" "$H/u03exe/rustnote.exe"
EXE_SAVE=$EXE; EXE="$H/u03exe/rustnote.exe"
run_case u03_start_assets PORT=3014 DATABASE_URL=assets.db
EXE=$EXE_SAVE

# ---- 問1: .env と環境変数の両方にあるとき
printf 'DATABASE_URL=libsql://from-dotenv.example\n' > "$RUN/.env"
run_case u03_q1_both PORT=3014 DATABASE_URL=libsql://from-env.example
run_case u03_q1_dotenv PORT=3014
rm -f "$RUN/.env"

# ---- エラー: lib.rs から pub mod flash; を消す・Secret を {} で表示・unsafe なしの set_var
cd "$H/rustnote" || exit 1
python - "$H/rustnote/src/lib.rs" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
assert s.count("pub mod flash;\n") == 1
open(p, "w", encoding="utf-8", newline="").write(s.replace("pub mod flash;\n", ""))
PY
$CAP u03_err_mod cargo build > /dev/null
cp "$REAL/src/lib.rs" src/lib.rs
for e in e03_display e03_setvar; do
  cp "$H/errs_u03/$e.rs" examples/
  $CAP "u03_err_${e#e03_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done

rm -rf "$RUN" "$H/u03exe"
check

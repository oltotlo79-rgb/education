#!/usr/bin/env bash
# 第13巻 総合演習2: 待ち受けの場所を DB より先に取る。直す前（教材）と直したあと（fixed）を、本物の実行ファイルで比べる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
O="$H/out"
export PYTHONIOENCODING=utf-8
# 直す前: 教材のコード（rust13/rustnote）の release、直したあと: fixed の release
(cd "$H/rustnote" && CARGO_TARGET_DIR="$H/target" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
(cd "$H/fixed" && CARGO_TARGET_DIR="$H/target_fixed" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
BEFORE="$H/target/release/rustnote.exe"
AFTER="$H/target_fixed/release/rustnote.exe"
RUN="$H/ex2run"
mask() { python "$H/mask13.py" | tr -d '\r'; }
files() { ls -1 "$RUN" | grep -v -E '^(pid|raw|stdout)$' | tr '\n' ' '; }
run_case() {  # 名前・実行ファイル・環境変数…
  local name=$1 exe=$2; shift 2
  rm -rf "$RUN"; mkdir -p "$RUN"
  ( cd "$RUN" && env "$@" "$exe" > "$RUN/stdout" 2> "$RUN/raw" & echo $! > "$RUN/pid" )
  for i in $(seq 1 20); do
    sleep 0.5
    if grep -q "RustNote を起動しました" "$RUN/raw" 2>/dev/null; then
      sleep 0.5
      echo "（/health: $(curl.exe -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3014/health)）" >> "$RUN/raw"
      taskkill //F //IM rustnote.exe > /dev/null 2>&1; break
    fi
    kill -0 "$(cat "$RUN/pid")" 2> /dev/null || break
  done
  sleep 1
  { mask < "$RUN/raw"; echo "（このあとのフォルダの中: $(files)）"; } | grep -v "^$" > "$O/$name.txt"
}

# PORT が数でない
run_case ex2_before_port "$BEFORE" PORT=abc DATABASE_URL=port.db
run_case ex2_after_port "$AFTER" PORT=abc DATABASE_URL=port.db
# ポートがほかのプログラムに使われている（python で 3014 を先に取っておく）
busy_case() {  # 名前・実行ファイル: 3014 を python で先に取ってから試す
  python -c "import socket, time; s = socket.socket(); s.bind(('127.0.0.1', 3014)); s.listen(); time.sleep(15)" &
  local busy=$!
  sleep 2
  local owner
  owner=$(netstat -ano | grep -c "127.0.0.1:3014 .*LISTENING")
  run_case "$1" "$2" PORT=3014 DATABASE_URL=busy.db
  echo "（試したとき、3014 で待ち受けていたもの: $owner 個）" >> "$O/$1.txt"
  wait $busy 2> /dev/null
  sleep 1
}
busy_case ex2_before_busy "$BEFORE"
busy_case ex2_after_busy "$AFTER"
# ふつうに起動
run_case ex2_after_ok "$AFTER" PORT=3014 DATABASE_URL=ok.db
rm -rf "$RUN"
echo done

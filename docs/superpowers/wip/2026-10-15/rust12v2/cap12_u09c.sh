#!/usr/bin/env bash
# 第12巻 UNIT 09: 1回のリクエストで、RustNote が sqld に何回 HTTP のリクエストを送っているかを数える。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
export PYTHONIOENCODING=utf-8
SQLD=ghcr.io/tursodatabase/libsql-server:latest
EXE="$H/u07/bin/v1/rustnote.exe"
P=http://127.0.0.1:18090
LOG="$H/u09_proxy.log"
taskkill //F //IM rustnote.exe > /dev/null 2>&1
docker rm -f sqld-u09 > /dev/null 2>&1
docker run -d --name sqld-u09 -p 18081:8080 "$SQLD" > /dev/null
sleep 3
rm -f "$LOG"; : > "$LOG"
python "$H/count_proxy.py" 28181 http://127.0.0.1:18081 "$LOG" &
PROXY=$!
sleep 1
( PORT=18090 DATABASE_URL=http://127.0.0.1:28181 "$EXE" > /dev/null 2> "$H/u09c.err" & )
for i in $(seq 1 100); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
JAR="$H/u09cjar"
curl.exe -s -o /dev/null -c "$JAR" -d "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$P/signup"
for k in $(seq 1 25); do curl.exe -s -o /dev/null -b "$JAR" -d "title=memo$k&body=" "$P/notes"; done
sleep 0.5
step() {  # 名前・curl の引数…
  local name=$1; shift
  local before after
  before=$(wc -l < "$LOG")
  local code
  code=$(curl.exe -s -o /dev/null -w '%{http_code}' "$@")
  sleep 0.5
  after=$(wc -l < "$LOG")
  echo "--- $name（状態 $code）: sqld へのリクエスト $((after - before)) 回"
  sed -n "$((before + 1)),${after}p" "$LOG" | nl -w2 -s'. '
}
{
  step "GET /health" "$P/health"
  step "GET /notes（ログイン中・25 件）" -b "$JAR" "$P/notes"
  step "POST /login" -d "email=alice%40example.com&password=correct+horse+battery" "$P/login"
} > "$O/u09_requests.txt"
taskkill //F //IM rustnote.exe > /dev/null 2>&1
kill $PROXY 2>/dev/null
docker rm -f sqld-u09 > /dev/null 2>&1
rm -f "$JAR"
echo done

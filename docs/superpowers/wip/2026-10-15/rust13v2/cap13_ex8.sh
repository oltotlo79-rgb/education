#!/usr/bin/env bash
# 第13巻 総合演習8: 守りのヘッダー。fixed の release で、curl と Chrome（CSP の違反・axe）で確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
O="$H/out"
export PYTHONIOENCODING=utf-8
(cd "$H/fixed" && CARGO_TARGET_DIR="$H/target_fixed" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
RUN="$H/ex8run"; rm -rf "$RUN"; mkdir -p "$RUN"
B=http://127.0.0.1:3014
( cd "$RUN" && PORT=3014 DATABASE_URL=ex8.db "$H/target_fixed/release/rustnote.exe" > "$RUN/server.log" 2>&1 & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null "$B/health" && break; done
hdr() { curl.exe -s -i "$@" | tr -d '\r' | grep -iE "^(HTTP|x-content|x-frame|referrer|content-security|location)"; }
{
  echo "--- GET /login"; hdr "$B/login"
  echo "--- GET /nope（404）"; hdr "$B/nope"
  echo "--- GET /health"; hdr "$B/health"
  echo "--- POST /login（失敗・401）"; hdr -d "email=a%40example.com&password=wrong-password" "$B/login"
  echo "--- POST /signup（成功・303）"; hdr -d "email=bob%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$B/signup"
} > "$O/ex8_headers.txt"
(cd "$H/u09lab" && node measure.mjs csp 2>&1 | tr -d '\r') > "$O/ex8_browser.txt"
taskkill //F //IM rustnote.exe > /dev/null 2>&1
sleep 1
rm -rf "$RUN"
echo done

#!/usr/bin/env bash
# 第13巻 総合演習8: topcoat dev で動かしたときは CSP を付けないことを確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
D="$H/ex8dev"; rm -rf "$D"; mkdir -p "$D"
cd "$H/fixed" || exit 1
( PORT=3016 DATABASE_URL="$D/dev.db" CARGO_TARGET_DIR="$H/target" topcoat dev > "$D/dev.log" 2>&1 & )
for i in $(seq 1 120); do sleep 1; curl.exe -s -o /dev/null http://127.0.0.1:3016/health && break; done
{
  echo "--- topcoat dev で動かした RustNote の GET /login"
  curl.exe -s -i http://127.0.0.1:3016/login | tr -d '\r' | grep -iE "^(HTTP|x-content|x-frame|referrer|content-security)"
  echo "--- 本文の <script>"
  curl.exe -s http://127.0.0.1:3016/login | grep -o '<script[^>]*>' | sed -E 's/127\.0\.0\.1:[0-9]+/127.0.0.1:（topcoat dev のポート）/'
} > "$H/out/ex8_dev.txt"
taskkill //F //IM topcoat.exe > /dev/null 2>&1; taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1
rm -rf "$D"
echo done

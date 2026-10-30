#!/usr/bin/env bash
# 第13巻 総合演習7: Argon2 の同時実行に上限を付ける前と後で、同時 48 のログインのメモリのピークを比べる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
O="$H/out"
export PYTHONIOENCODING=utf-8
(cd "$H/rustnote" && CARGO_TARGET_DIR="$H/target" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
(cd "$H/fixed" && CARGO_TARGET_DIR="$H/target_fixed" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
URL=http://127.0.0.1:3014
LOGIN="email=alice%40example.com&password=correct+horse+battery"
SIGNUP="$LOGIN&password_confirm=correct+horse+battery"
peak_mb() { powershell -NoProfile -c "[int]((Get-Process rustnote).PeakWorkingSet64/1MB)" | tr -d '\r'; }
measure() {  # 名前・実行ファイル
  local name=$1 exe=$2 run="$H/ex7run"
  rm -rf "$run"; mkdir -p "$run"
  ( cd "$run" && PORT=3014 DATABASE_URL=ex7.db "$exe" > "$run/server.log" 2>&1 & )
  for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null "$URL/health" && break; done
  curl.exe -s -o /dev/null -d "$SIGNUP" "$URL/signup"
  {
    echo "起動して登録したあと: $(peak_mb) MiB"
    oha --no-tui -n 480 -c 48 -m POST -T application/x-www-form-urlencoded -d "$LOGIN" "$URL/login" | tr -d '\r' | grep -E "Success rate|Requests/sec|Slowest|Fastest|Average|\[303\]"
    echo "同時 48 のログインのあと: $(peak_mb) MiB"
  } > "$O/$name.txt" 2>&1
  taskkill //F //IM rustnote.exe > /dev/null 2>&1
  sleep 1
  rm -rf "$run"
}
measure ex7_before "$H/target/release/rustnote.exe"
measure ex7_after "$H/target_fixed/release/rustnote.exe"
echo done

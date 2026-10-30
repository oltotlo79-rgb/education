#!/usr/bin/env bash
# 第9巻 UNIT 10 の「初回起動」の流れを、新しい DB で通して記録する。
# 使い方: bash run_u10.sh
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
cd "$S/rust09/book/rustnote" || exit 1
export CARGO_TARGET_DIR="$S/target11"
M() { PYTHONIOENCODING=utf-8 python "$S/rust09/m9.py"; }
O="$S/rust09/out"
C=/c/Windows/System32/curl.exe
B="$CARGO_TARGET_DIR/debug/rustnote.exe"
D="$CARGO_TARGET_DIR/debug/examples/u10_doctor.exe"

cargo build -q --bin rustnote --example u10_doctor || exit 1
rm -f first.db first.db-*

# 1. 束を作る（--bin を付けない、README と同じ書き方）
{ echo '### topcoat asset bundle'; topcoat asset bundle 2>&1; echo "exit=$?"; } | M > "$O/u10_run1_bundle.txt"

# 2. 起動の前に診断（新しい DB）
{ echo '### $env:DATABASE_URL = "first.db"; cargo run --example u10_doctor'; DATABASE_URL=first.db "$D" 2>&1; echo "exit=$?"; echo '### Test-Path first.db'; [ -f first.db ] && echo True || echo False; } | M > "$O/u10_run2_doctor.txt"

# 3. 起動（別の窓）
DATABASE_URL=first.db "$B" > "$O/raw_u10_server.log" 2>&1 &
SERVER=$!
# 起動を待つ（/health はアクセスログに出ないので、待つのにも使える）
$C -s --retry 20 --retry-connrefused --retry-delay 1 -o /dev/null http://127.0.0.1:3000/health

# 4. 外から確かめる
{ echo '### curl.exe -i http://127.0.0.1:3000/health'; $C -s -i http://127.0.0.1:3000/health; echo; } | M > "$O/u10_run4_health.txt"

# 5. path を間違えた確かめ
{ echo '### curl.exe -s -o NUL -w "%{http_code}\n" http://127.0.0.1:3000/healthz'; $C -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:3000/healthz; } | M > "$O/u10_run5_healthz.txt"

# 6. 動かしたまま診断（読み違えの例）と、PORT を変えた診断
{ echo '### $env:DATABASE_URL = "first.db"; cargo run --example u10_doctor   # RustNote を動かしたまま'; DATABASE_URL=first.db "$D" 2>&1; echo "exit=$?"; } | M > "$O/u10_run6_doctor_busy.txt"
{ echo '### $env:PORT = "3001"; cargo run --example u10_doctor'; DATABASE_URL=first.db PORT=3001 "$D" 2>&1; echo "exit=$?"; } | M > "$O/u10_run6_doctor_3001.txt"

# 7. /health を5回送ってから、ログイン画面を1回開く（ログに何行出るか）
{ echo '### 1..5 | ForEach-Object { curl.exe -s http://127.0.0.1:3000/health }; curl.exe -s -o NUL http://127.0.0.1:3000/login'; for i in 1 2 3 4 5; do $C -s http://127.0.0.1:3000/health; echo; done; $C -s -o /dev/null http://127.0.0.1:3000/login; } | M > "$O/u10_run7_client.txt"

kill $SERVER
wait $SERVER 2>/dev/null
{ echo '### （RustNote の窓に出た表示。Ctrl+C で止めるまで）'; cat "$O/raw_u10_server.log"; } | M > "$O/u10_run3_server.txt"
rm -f "$O/raw_u10_server.log" first.db first.db-*
for f in "$O"/u10_run*.txt; do echo "== $(basename "$f")"; cat "$f"; done

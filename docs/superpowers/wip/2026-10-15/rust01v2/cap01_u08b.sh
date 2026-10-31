#!/usr/bin/env bash
# 第1巻 UNIT 08 の続き: topcoat dev の表示と、保存したときの作り直し。topcoat asset bundle のあとの cargo run。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
export RUSTUP_HOME='C:\Users\oltot\rsb\rustup' CARGO_HOME='C:\Users\oltot\rsb\cargo'
export PATH="/c/Users/oltot/rsb/cargo/bin:$(echo "$PATH" | tr ':' '\n' | grep -v cargo | paste -sd:)"
mask() { python "$H/mask01.py"; }
wait_port() { for i in $(seq 1 240); do curl.exe -s -o /dev/null "http://127.0.0.1:$1/" && return 0; sleep 0.5; done; return 1; }
A=/c/Users/oltot/rust-study/hello-topcoat
cd "$A" || exit 1
taskkill //F //IM topcoat.exe > /dev/null 2>&1; taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1; taskkill //F //IM hello-topcoat.topcoat-dev.exe > /dev/null 2>&1
cp "$H/u08_app_snapshot/src/app.rs" "$A/src/app.rs"

# ---- 例8-4: topcoat dev
( topcoat dev > "$H/u08_dev.raw" 2>&1 & )
wait_port 3000
sleep 3
curl.exe -s http://127.0.0.1:3000/ | grep -o '<h1>[^<]*</h1>' > "$O/u08_dev_h1_before.txt"
# 見出しを書き換えて保存する
python - "$A/src/app.rs" <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
old = '<h1>"hello-topcoat"</h1>'
assert s.count(old) == 1
open(p, "w", encoding="utf-8", newline="\n").write(s.replace(old, '<h1>"こんにちは、Topcoat"</h1>'))
PY
for i in $(seq 1 120); do curl.exe -s http://127.0.0.1:3000/ 2>/dev/null | grep -q "こんにちは" && break; sleep 1; done
curl.exe -s http://127.0.0.1:3000/ | grep -o '<h1>[^<]*</h1>' > "$O/u08_dev_h1_after.txt"
sleep 1
taskkill //F //IM topcoat.exe > /dev/null 2>&1; taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1; taskkill //F //IM hello-topcoat.topcoat-dev.exe > /dev/null 2>&1
sleep 1
mask < "$H/u08_dev.raw" > "$O/u08_dev.txt"
cp "$H/u08_app_snapshot/src/app.rs" "$A/src/app.rs"

# ---- 例8-3 の続き: 束を作ってから cargo run
topcoat asset bundle 2>&1 | mask > "$O/u08_bundle.txt"
ls target/debug/assets > "$O/u08_bundle_ls.txt"
( cargo run > "$H/u08_run2.raw" 2>&1 & )
wait_port 3000
curl.exe -s -i http://127.0.0.1:3000/ | tr -d '\r' | head -3 > "$O/u08_run2_curl.txt"
# ---- エラー: 同じポートで、もう1つ動かす
./target/debug/hello-topcoat.exe > "$H/u08_busy.raw" 2>&1; echo "exit=$?" >> "$H/u08_busy.raw"
mask < "$H/u08_busy.raw" > "$O/u08_port_busy.txt"
taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1
sleep 1
mask < "$H/u08_run2.raw" > "$O/u08_run2.txt"
echo done

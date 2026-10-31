#!/usr/bin/env bash
# 第1巻 UNIT 08 の出力を取る（Topcoat 0.11 と CLI）。
# topcoat-cli を入れる所と、新しいアプリは rsb の Rust で、RustNote はいつもの Rust で試す。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
W=/c/Users/oltot/rust-study
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location ~; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
psbx() { psb "$1; 'exit=' + \$LASTEXITCODE"; }
wait_port() { for i in $(seq 1 240); do curl.exe -s -o /dev/null "http://127.0.0.1:$1/" && return 0; sleep 0.5; done; return 1; }
cd ~ || exit 1
mkdir -p "$W"

# ---- 例8-1: topcoat-cli を入れる（版を決めて、Cargo.lock のとおりに）
start=$(date +%s)
psbx "cargo install topcoat-cli --version 0.11.0 --locked" > "$O/u08_install.txt"
echo "（かかった時間: $(( $(date +%s) - start )) 秒）" >> "$O/u08_install.txt"
psb "topcoat --version; where.exe topcoat; topcoat --help" > "$O/u08_help.txt"

# ---- 例8-2: 新しいアプリを作る
rm -rf "$W/hello-topcoat"
psbx "Set-Location C:\Users\oltot\rust-study; topcoat new hello-topcoat --minimal --no-interactive" > "$O/u08_new.txt"
( cd "$W/hello-topcoat" && find . -type f -not -path "./.git/*" -not -path "./target/*" | sort ) > "$O/u08_tree.txt"
cp -r "$W/hello-topcoat" "$H/u08_app_snapshot" 2>/dev/null

# ---- 例8-3: cargo run で動かして、ブラウザの代わりに curl で開く
taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1
start=$(date +%s)
( psb "Set-Location C:\Users\oltot\rust-study\hello-topcoat; cargo run" > "$O/u08_run.txt" & )
wait_port 3000
echo "（cargo run から開けるまで: $(( $(date +%s) - start )) 秒）" > "$O/u08_run_time.txt"
curl.exe -s -i "http://127.0.0.1:3000/" | tr -d '\r' > "$O/u08_curl.txt"
# ---- エラー: 同じポートで、もう1つ動かす
psbx "Set-Location C:\Users\oltot\rust-study\hello-topcoat; .\target\debug\hello-topcoat.exe" > "$O/u08_port_busy.txt"
taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1
sleep 2

# ---- 例8-4: topcoat dev（開発用のサーバー）
( psb "Set-Location C:\Users\oltot\rust-study\hello-topcoat; topcoat dev" > "$O/u08_dev.txt" & )
wait_port 3000
sleep 3
curl.exe -s "http://127.0.0.1:3000/" | tr -d '\r' > "$O/u08_dev_page.txt"
taskkill //F //IM topcoat.exe > /dev/null 2>&1
taskkill //F //IM hello-topcoat.exe > /dev/null 2>&1
sleep 2

# ---- エラー: 古い Rust で Topcoat 0.11 を使う（いつもの Rust の 1.90.0）
REAL='C:\Users\oltot\Documents\git-projects\def\Rust\rustnote'
T=$(cygpath -w "$H/t08old")
powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location $REAL; \$env:CARGO_TARGET_DIR = '$T'; cargo +1.90.0 check; 'exit=' + \$LASTEXITCODE" 2>&1 | python "$S/maskx.py" | tr -d '\r' > "$O/u08_old_rust.txt"
echo done

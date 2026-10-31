#!/usr/bin/env bash
# 第1巻 UNIT 09 の出力を取る（libSQL の確認）。Turso CLI は入れず、同じ libSQL のサーバー（sqld）を Docker で動かす。
# 本文に載せるコマンドは、本文と同じ形で PowerShell から動かす。RustNote が止まらないときに備えて、裏で動かして止める。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
cd ~ || exit 1
mask() { python "$H/mask01.py"; }
REAL='C:\Users\oltot\Documents\git-projects\def\Rust\rustnote'
T=$(cygpath -w "$H/t06build")
TS=$(cygpath -w "$H/t09stack")
ps() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location ~; $1" 2>&1 | mask; }
# いつもの Rust で、RustNote のフォルダから動かす PowerShell（target は練習用の場所）
prn() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location $REAL; \$env:CARGO_TARGET_DIR = '$T'; $1" 2>&1 | mask; }
wait_url() { for i in $(seq 1 360); do curl.exe -s -o /dev/null "$1" && return 0; sleep 0.5; done; return 1; }
stop_rn() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 2; }
stop_rn
docker rm -f u09sqld > /dev/null 2>&1

# ---- 例9-2: 手元で libSQL のサーバーを動かす
ps "docker run -d --name u09sqld -p 8080:8080 ghcr.io/tursodatabase/libsql-server:latest" > "$O/u09_sqld_run.txt"
wait_url http://127.0.0.1:8080/health
ps "docker ps --filter name=u09sqld --format '{{.Image}}  {{.Status}}  {{.Ports}}'; curl.exe -s -o NUL -w '%{http_code}' http://127.0.0.1:8080/health; ''; curl.exe -s http://127.0.0.1:8080/version" > "$O/u09_sqld.txt"

# ---- エラー1: 開発用のビルド（cargo run）だと、スタックがあふれる
prn "topcoat asset bundle" > "$O/u09_bundle.txt"
( prn "\$env:DATABASE_URL = 'http://127.0.0.1:8080'; cargo run; 'exit=' + \$LASTEXITCODE" > "$O/u09_debug_overflow.txt" & )
sleep 25; stop_rn

# ---- 例9-3: release で動かして、そのサーバーにつなぐ
prn "topcoat asset bundle --release" > "$O/u09_bundle_release.txt"
( prn "\$env:DATABASE_URL = 'http://127.0.0.1:8080'; cargo run --release" > "$O/u09_rustnote_log.txt" & )
wait_url http://127.0.0.1:3000/health
ps "curl.exe -s -i http://127.0.0.1:3000/health; ''; curl.exe -s -o NUL -w '%{http_code} %{redirect_url}' -d 'email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery' http://127.0.0.1:3000/signup" > "$O/u09_rustnote_check.txt"
cp "$H/u09_sql.ps1" ~/rust-study/u09_sql.ps1
ps "& ~\rust-study\u09_sql.ps1" > "$O/u09_http_sql.txt"
stop_rn

# ---- エラー1 の直し方の2つ目: main のスタックを 8 MiB にして、開発用のビルドで動かす
( powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location $REAL; \$env:CARGO_TARGET_DIR = '$TS'; \$env:DATABASE_URL = 'http://127.0.0.1:8080'; \$env:RUSTFLAGS = '-C link-arg=/STACK:8388608'; cargo run" 2>&1 | mask > "$O/u09_stack8.txt" & )
wait_url http://127.0.0.1:3000/health
echo "GET /health: $(curl.exe -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3000/health)" > "$O/u09_stack8_health.txt"
stop_rn

# ---- エラー2: http:// を付け忘れた（起動してしまっても止められるよう、15 秒後に止める）
( prn "\$env:DATABASE_URL = '127.0.0.1:8080'; cargo run --release; 'exit=' + \$LASTEXITCODE" > "$O/u09_no_scheme.txt" & )
sleep 15
echo "--- 15 秒後の http://127.0.0.1:3000/health: $(curl.exe -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3000/health)" > "$O/u09_no_scheme_health.txt"
stop_rn
ls -la /c/Users/oltot/Documents/git-projects/def/Rust/rustnote | grep -v "^d" | awk '{print $NF}' > "$O/u09_no_scheme_files.txt"
# ---- エラー3: Turso の URL なのに、トークンが無い
( prn "\$env:DATABASE_URL = 'libsql://rustnote-example.turso.io'; cargo run --release; 'exit=' + \$LASTEXITCODE" > "$O/u09_no_token.txt" & )
sleep 15; stop_rn

# ---- 問3: コンテナを消して動かし直すと
ps "docker rm -f u09sqld | Out-Null; docker run -d --name u09sqld -p 8080:8080 ghcr.io/tursodatabase/libsql-server:latest | Out-Null"
wait_url http://127.0.0.1:8080/health
ps "& ~\rust-study\u09_sql.ps1" > "$O/u09_q3.txt"
docker rm -f u09sqld > /dev/null 2>&1
echo done

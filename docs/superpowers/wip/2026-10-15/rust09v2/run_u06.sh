#!/usr/bin/env bash
# 第9巻 UNIT 06: RustNote の起動の順番を、いろいろな設定で確かめる
set -u
S="$(cd "$(dirname "$0")/.." && pwd)"
N="$S/rust09/book/rustnote"
O="$S/rust09/out"
export CARGO_TARGET_DIR="$S/target11"
C=/c/Windows/System32/curl.exe
M() { PYTHONIOENCODING=utf-8 python "$S/rust09/m9.py"; }
stop() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
cd "$N" || exit 1
rm -f first.db first.db-* .env
stop

# 1. ふつうに起動（新しい DB）→ もう一度起動
{
  echo '### $env:DATABASE_URL = "first.db"; cargo run'
  (DATABASE_URL=first.db cargo run -q > "$O/u06_run1.raw" 2>&1 &)
  sleep 4
  cat "$O/u06_run1.raw"
  echo '### （2つ目の窓から）curl.exe -s http://127.0.0.1:3000/health'
  $C -s http://127.0.0.1:3000/health; echo
} | M > "$O/u06_first.txt"
stop
{
  echo '### （止めてから、同じ DB でもう一度）cargo run'
  (DATABASE_URL=first.db cargo run -q > "$O/u06_run2.raw" 2>&1 &)
  sleep 4
  cat "$O/u06_run2.raw"
} | M > "$O/u06_second.txt"
stop

# 2. Turso の URL なのにトークンが無い
{
  echo '### $env:DATABASE_URL = "libsql://example.turso.io"; cargo run'
  DATABASE_URL=libsql://example.turso.io cargo run -q 2>&1; echo "exit=$?"
} | M > "$O/u06_err_token.txt"

# 3. 無いフォルダの中の DB
{
  echo '### $env:DATABASE_URL = "no_such_dir\first.db"; cargo run'
  DATABASE_URL='no_such_dir\first.db' cargo run -q 2>&1; echo "exit=$?"
} | M > "$O/u06_err_dbpath.txt"

# 4. アセットの束が無い（DB のファイルは作られるか）
topcoat asset clean > /dev/null 2>&1
{
  echo '### topcoat asset clean のあとで $env:DATABASE_URL = "nobundle.db"; cargo run'
  DATABASE_URL=nobundle.db cargo run -q 2>&1; echo "exit=$?"
  echo '### Test-Path nobundle.db'
  if [ -e nobundle.db ]; then echo True; else echo False; fi
} | M > "$O/u06_err_bundle.txt"
topcoat asset bundle --bin rustnote > /dev/null 2>&1

# 5. .env と、環境変数の両方に PORT がある
printf 'PORT=3010\nDATABASE_URL=first.db\n' > .env
{
  echo '### .env に PORT=3010 を書いて cargo run'
  (cargo run -q > "$O/u06_run3.raw" 2>&1 &)
  sleep 4
  cat "$O/u06_run3.raw"
} | M > "$O/u06_dotenv1.txt"
stop
{
  echo '### .env はそのままで、$env:PORT = "3011"; cargo run'
  (PORT=3011 cargo run -q > "$O/u06_run4.raw" 2>&1 &)
  sleep 4
  cat "$O/u06_run4.raw"
} | M > "$O/u06_dotenv2.txt"
stop
rm -f .env

# 6. ポートがもう使われている
(DATABASE_URL=first.db cargo run -q > /dev/null 2>&1 &)
sleep 4
{
  echo '### （1つ目が動いている間に、2つ目の窓で）cargo run'
  DATABASE_URL=first.db cargo run -q 2>&1; echo "exit=$?"
} | M > "$O/u06_err_port.txt"
stop
rm -f first.db first.db-* nobundle.db nobundle.db-*
tasklist | grep -i rustnote.exe || echo "サーバーは残っていない"

#!/usr/bin/env bash
# 第12巻 UNIT 06 の出力を取る（health check と起動の順番）。
# Turso の代わりに、手元の Docker で libSQL のサーバー（sqld）を動かし、RustNote から http:// でつなぐ。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
export PYTHONIOENCODING=utf-8
IMG=rustnote:u02
SQLD=ghcr.io/tursodatabase/libsql-server:latest
P=http://127.0.0.1:18080
cleanup() { docker rm -f rn-app sqld > /dev/null 2>&1; }
cleanup
docker network rm rn-net > /dev/null 2>&1
docker network create rn-net > /dev/null
docker pull -q "$SQLD" > /dev/null
docker image inspect "$SQLD" --format 'sqld のイメージ: {{.Architecture}}・{{index .Config.Labels "org.opencontainers.image.version"}}' > "$O/u06_sqld_image.txt" 2>&1
status() { curl.exe -s -o /dev/null -w "%{http_code}" "$P/health"; }
body() { curl.exe -s "$P/health"; }
since() { python -c "import sys, time; print(f'{time.time() - float(sys.argv[1]):.1f}')" "$1"; }

# ---- 例6-1: sqld を起動し、RustNote を http://sqld:8080 につなぐ
docker run -d --name sqld --network rn-net "$SQLD" > /dev/null
sleep 2
start=$(python -c "import time; print(time.time())")
docker run -d --name rn-app --network rn-net -p 18080:8080 -e DATABASE_URL=http://sqld:8080 "$IMG" > /dev/null
first=""
for i in $(seq 1 200); do
  code=$(status)
  if [ "$code" = "200" ]; then first=$(since "$start"); break; fi
  sleep 0.1
done
{
  echo "起動してから /health が 200 になるまで: 約 $first 秒"
  echo "/health: $(status) $(body)"
  echo "--- RustNote のログ"
  docker logs rn-app 2>&1 | tr -d '\r'
} > "$O/u06_start.txt"

# ---- 例6-2: DB（sqld）を止める → /health は？ → 動かし直す
docker stop sqld > /dev/null
sleep 1
{
  echo "--- sqld を止めたあと"
  for k in 1 2 3; do echo "/health: $(status) $(body)"; sleep 1; done
  echo "--- そのときの / （トップページ）: $(curl.exe -s -o /dev/null -w "%{http_code}" "$P/")"
} > "$O/u06_db_down.txt"
docker start sqld > /dev/null
for i in $(seq 1 100); do [ "$(status)" = "200" ] && break; sleep 0.2; done
{
  echo "--- sqld を動かし直したあと"
  echo "/health: $(status) $(body)"
  echo "--- RustNote のログ（最後の5行）"
  docker logs rn-app 2>&1 | tr -d '\r' | tail -5
} > "$O/u06_db_back.txt"
docker rm -f rn-app > /dev/null

# ---- エラー: DB が動いていないときに RustNote を起動する（起動の順番）
docker stop sqld > /dev/null
docker run --name rn-app --network rn-net -e DATABASE_URL=http://sqld:8080 "$IMG" > "$H/u06_nodb.out" 2>&1
echo "（終了コード: $?）" >> "$H/u06_nodb.out"
tr -d '\r' < "$H/u06_nodb.out" > "$O/u06_err_nodb.txt"
docker rm -f rn-app > /dev/null 2>&1
# Fly.io のように「止まったら動かし直す」（--restart on-failure:3）で、DB があとから来たら？
docker run -d --name rn-app --network rn-net -p 18080:8080 --restart on-failure:5 -e DATABASE_URL=http://sqld:8080 "$IMG" > /dev/null
sleep 4
docker start sqld > /dev/null
for i in $(seq 1 150); do [ "$(status)" = "200" ] && break; sleep 0.2; done
{
  echo "/health: $(status) $(body)"
  echo "動かし直した回数: $(docker inspect rn-app --format '{{.RestartCount}}')"
  echo "--- RustNote のログ"
  docker logs rn-app 2>&1 | tr -d '\r'
} > "$O/u06_restart.txt"

cleanup
docker network rm rn-net > /dev/null 2>&1
rm -f "$H/u06_nodb.out"
echo done

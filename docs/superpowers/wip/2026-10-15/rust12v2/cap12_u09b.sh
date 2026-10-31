#!/usr/bin/env bash
# 第12巻 UNIT 09 のエラーの例: DB までがとても遅いと、ヘルスチェックの時間切れ（5 秒）に届くか。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
IMG=rustnote:u02
SQLD=ghcr.io/tursodatabase/libsql-server:latest
TOXI=ghcr.io/shopify/toxiproxy:latest
P=http://127.0.0.1:18080
API=http://127.0.0.1:18474
cleanup() { docker rm -f rn-app sqld toxiproxy > /dev/null 2>&1; }
cleanup; docker network rm rn-net > /dev/null 2>&1; docker network create rn-net > /dev/null
docker run -d --name sqld --network rn-net "$SQLD" > /dev/null
docker run -d --name toxiproxy --network rn-net -p 18474:8474 "$TOXI" > /dev/null
sleep 3
curl.exe -s -o /dev/null -X POST "$API/proxies" -d '{"name":"sqld","listen":"0.0.0.0:28080","upstream":"sqld:8080"}'
docker run -d --name rn-app --network rn-net -p 18080:8080 -e DATABASE_URL=http://toxiproxy:28080 "$IMG" > /dev/null
for i in $(seq 1 100); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
{
  for ms in 1500 3000; do
    curl.exe -s -o /dev/null -X DELETE "$API/proxies/sqld/toxics/lag"
    curl.exe -s -o /dev/null -X POST "$API/proxies/sqld/toxics" -d "{\"name\":\"lag\",\"type\":\"latency\",\"stream\":\"downstream\",\"attributes\":{\"latency\":$ms,\"jitter\":0}}"
    out=$(curl.exe -s -o /dev/null -m 5 -w '%{http_code} %{time_total}' "$P/health"); code=$?
    echo "遅れ（片道）$ms ms: 状態 ${out% *}・かかった時間 ${out#* } 秒・curl の終了コード $code"
  done
} > "$O/u09_timeout.txt"
cleanup; docker network rm rn-net > /dev/null 2>&1
echo done

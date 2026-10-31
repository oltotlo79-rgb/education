#!/usr/bin/env bash
# 第12巻 UNIT 09 の出力を取る（台数・地域・費用・Turso との距離）。
# RustNote と sqld の間に toxiproxy をはさみ、DB までの遅れ（片道）を 0・10・50・100 ms にして、応答の時間を測る。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
export PYTHONIOENCODING=utf-8
IMG=rustnote:u02
SQLD=ghcr.io/tursodatabase/libsql-server:latest
TOXI=ghcr.io/shopify/toxiproxy:latest
P=http://127.0.0.1:18080
API=http://127.0.0.1:18474
cleanup() { docker rm -f rn-app sqld toxiproxy > /dev/null 2>&1; }
cleanup
docker network rm rn-net > /dev/null 2>&1
docker network create rn-net > /dev/null
docker pull -q "$SQLD" > /dev/null; docker pull -q "$TOXI" > /dev/null
docker image inspect "$TOXI" --format 'toxiproxy のイメージ: {{.Architecture}}' > "$O/u09_toxi_image.txt"
docker run -d --name sqld --network rn-net "$SQLD" > /dev/null
docker run -d --name toxiproxy --network rn-net -p 18474:8474 "$TOXI" > /dev/null
sleep 3
curl.exe -s -o /dev/null -X POST "$API/proxies" -d '{"name":"sqld","listen":"0.0.0.0:28080","upstream":"sqld:8080"}'
docker run -d --name rn-app --network rn-net -p 18080:8080 -e DATABASE_URL=http://toxiproxy:28080 "$IMG" > /dev/null
for i in $(seq 1 100); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
JAR="$H/u09jar"
curl.exe -s -o /dev/null -c "$JAR" -d "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$P/signup"
for k in $(seq 1 25); do curl.exe -s -o /dev/null -b "$JAR" -d "title=memo$k&body=" "$P/notes"; done
LOGIN="email=alice%40example.com&password=correct+horse+battery"
median() { python -c "import statistics, sys; print(f'{statistics.median(float(x) for x in sys.argv[1:]) * 1000:.0f}')" "$@"; }
measure() {  # 名前・curl の引数…: 7 回の中央値（ミリ秒）
  local ts=""
  for i in 1 2 3 4 5 6 7; do ts="$ts $(curl.exe -s -o /dev/null -w '%{time_total}' "$@")"; done
  median $ts
}
{
  echo "遅れ（片道）| GET /health | GET /notes（ログイン中・25 件）| POST /login"
  for ms in 0 10 50 100; do
    curl.exe -s -o /dev/null -X DELETE "$API/proxies/sqld/toxics/lag"
    if [ "$ms" != 0 ]; then
      curl.exe -s -o /dev/null -X POST "$API/proxies/sqld/toxics" -d "{\"name\":\"lag\",\"type\":\"latency\",\"stream\":\"downstream\",\"attributes\":{\"latency\":$ms,\"jitter\":0}}"
    fi
    h=$(measure "$P/health")
    n=$(measure -b "$JAR" "$P/notes")
    l=$(measure -d "$LOGIN" "$P/login")
    echo "$ms ms | $h ms | $n ms | $l ms"
  done
} > "$O/u09_latency.txt"
curl.exe -s -o /dev/null -X DELETE "$API/proxies/sqld/toxics/lag"

# 1回の GET /notes で、DB に何回問い合わせているか（sqld へのリクエストの数を toxiproxy の前後で数える代わりに、遅れからの割り算で見積もる）
docker logs rn-app 2>&1 | tr -d '\r' | tail -3 > "$O/u09_app_log.txt"

# 比べるために、DB がコンテナの中のファイル（遅れ 0）のとき
docker rm -f rn-app > /dev/null
docker run -d --name rn-app -p 18080:8080 "$IMG" > /dev/null
for i in $(seq 1 100); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
curl.exe -s -o /dev/null -c "$JAR" -d "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$P/signup"
for k in $(seq 1 25); do curl.exe -s -o /dev/null -b "$JAR" -d "title=memo$k&body=" "$P/notes"; done
{
  echo "ローカルのファイル（/data/rustnote.db）| $(measure "$P/health") ms | $(measure -b "$JAR" "$P/notes") ms | $(measure -d "$LOGIN" "$P/login") ms"
} > "$O/u09_local.txt"

cleanup
docker network rm rn-net > /dev/null 2>&1
rm -f "$JAR"
echo done

#!/usr/bin/env bash
# 第12巻 UNIT 08 の出力を取る（deploy のあとの動作の確かめ・ログ）。Fly.io の代わりに、手元のコンテナで確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
export PYTHONIOENCODING=utf-8
IMG=rustnote:u02
P=http://127.0.0.1:18080
SMOKE="$(cygpath -w "$H/smoke.ps1")"
docker rm -f rn-u08 > /dev/null 2>&1
smoke() { powershell -NoProfile -ExecutionPolicy Bypass -File "$SMOKE" -Base "$1" 2>&1 | tr -d '\r'; echo "（終了コード: ${PIPESTATUS[0]}）"; }

# ---- 例8-1: ふつうに動いているコンテナに、スモークテストをかける
docker run -d --name rn-u08 -p 18080:8080 "$IMG" > /dev/null
for i in $(seq 1 50); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
smoke "$P" > "$O/u08_smoke_ok.txt"
docker logs rn-u08 2>&1 | tr -d '\r' > "$O/u08_logs.txt"
docker logs --since 0 rn-u08 2>&1 | tr -d '\r' | grep -c -- "->" > "$O/u08_log_count.txt"
docker rm -f rn-u08 > /dev/null

# ---- 例8-2: こわれた公開（HOST=127.0.0.1 で、外から届かない）に、スモークテストをかける
docker run -d --name rn-u08 -p 18080:8080 -e HOST=127.0.0.1 "$IMG" > /dev/null
sleep 3
smoke "$P" > "$O/u08_smoke_broken.txt"
docker rm -f rn-u08 > /dev/null

# ---- 例8-3: 古い CSS の束のまま（UNIT 03）でも、スモークテストは通ってしまう？ → CSS の URL も確かめる
docker run -d --name rn-u08 -p 18080:8080 "$IMG" > /dev/null
for i in $(seq 1 50); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
HREF=$(curl.exe -s "$P/" | grep -o '/_topcoat/assets/[^"]*')
{
  echo "トップの <link> の CSS: $HREF"
  echo "その CSS の状態: $(curl.exe -s -o /dev/null -w '%{http_code}' "$P$HREF")"
} > "$O/u08_css.txt"
docker rm -f rn-u08 > /dev/null
echo done

#!/usr/bin/env bash
# Smoke test: run the built probe binary and exercise it with curl.
# Usage (Git Bash):  cargo build && bash scripts/smoke.sh
set -u
cd "$(dirname "$0")/.."
PORT=${PORT:-3100}
BASE="http://127.0.0.1:$PORT"
JAR=$(mktemp)

PORT=$PORT ./target/debug/topcoat-probe.exe > smoke-server.log 2>&1 &
SERVER_PID=$!
for i in $(seq 1 30); do curl -s -o /dev/null "$BASE/api/health" && break; sleep 0.5; done

hr() { echo; echo "===== $*"; }

hr "GET / (hello page)"
curl -s -i "$BASE/"

hr "GET /api/health (JSON)"
curl -s -i "$BASE/api/health"

hr "GET /notes while logged out -> redirect to /login"
curl -s -i "$BASE/notes" | head -5

hr "GET /api/notes while logged out -> 401"
curl -s -i "$BASE/api/notes" | head -3

hr "POST /login wrong password -> 401 + form re-rendered"
curl -s -i -X POST -d "name=alice&password=nope" "$BASE/login" | sed -n '1p;/Invalid/p'

hr "POST /login correct -> 303 + Set-Cookie"
curl -s -i -c "$JAR" -X POST -d "name=alice&password=secret" "$BASE/login" | head -8
# (curl 8.x also sends this Secure cookie back to 127.0.0.1/localhost with
# `-b "$JAR"`; here we send it explicitly to show the raw Cookie header.)
TOKEN=$(grep -E "__Host-session" "$JAR" | awk '{print $7}')
COOKIE="Cookie: __Host-session=$TOKEN"
echo "token: $TOKEN"

hr "POST /notes (form) -> 303 Location: /notes/1"
curl -s -i -H "$COOKIE" -X POST --data-urlencode "title=First <note>" --data-urlencode "body=<script>alert(1)</script> & more" "$BASE/notes" | head -5

hr "GET /notes/1 (path param page, escaped body)"
curl -s -i -H "$COOKIE" "$BASE/notes/1"

hr "GET /notes/new (static segment beats {note_id})"
curl -s -H "$COOKIE" "$BASE/notes/new" | grep -o "<h1>[^<]*</h1>"

hr "GET /notes/abc (module_param error = not_found -> branded 404 via error_boundary)"
curl -s -i -H "$COOKIE" "$BASE/notes/abc" | sed -n '1p;/404/p'

hr "GET /notes/999 (ok_or_not_found -> 404)"
curl -s -i -H "$COOKIE" "$BASE/notes/999" | sed -n '1p'

hr "GET /does/not/exist (not_found! catch-all -> branded 404)"
curl -s -i "$BASE/does/not/exist" | sed -n '1p;/404/p'

hr "GET /notes?q=First (query params)"
curl -s -H "$COOKIE" "$BASE/notes?q=First" | grep -o "<ul>.*</ul>"

hr "GET /notes?q= (empty value -> None)"
curl -s -H "$COOKIE" "$BASE/notes?q=" | grep -o "<ul>.*</ul>"

hr "POST /notes/1/edit with empty title -> 422"
curl -s -i -H "$COOKIE" -X POST -d "title=&body=x" "$BASE/notes/1/edit" | sed -n '1p;/errors/p'

hr "POST /notes/1/edit valid -> 303"
curl -s -i -H "$COOKIE" -X POST -d "title=Edited&body=y" "$BASE/notes/1/edit" | head -4

hr "POST /api/notes (JSON) -> 201"
curl -s -i -H "$COOKIE" -H "Content-Type: application/json" -d '{"title":"from api"}' "$BASE/api/notes"

hr "GET /api/notes (JSON list)"
curl -s -H "$COOKIE" "$BASE/api/notes"

hr "POST /notes with JSON content-type (Form extractor) -> 400"
curl -s -i -H "$COOKIE" -H "Content-Type: application/json" -d '{}' "$BASE/notes" | head -1

hr "POST /notes missing field -> 400"
curl -s -i -H "$COOKIE" -X POST -d "title=x" "$BASE/notes" | sed -n '1p;$p'

hr "Cross-site POST (Sec-Fetch-Site: cross-site) -> 403 (OriginPolicy)"
curl -s -i -H "$COOKIE" -H "Sec-Fetch-Site: cross-site" -X POST -d "title=x&body=y" "$BASE/notes" | head -1

hr "Cross-origin POST (Origin != Host, no Sec-Fetch-Site) -> 403"
curl -s -i -H "$COOKIE" -H "Origin: https://evil.example" -X POST -d "title=x&body=y" "$BASE/notes" | head -1

hr "Same-origin POST (Sec-Fetch-Site: same-origin) -> 303"
curl -s -i -H "$COOKIE" -H "Sec-Fetch-Site: same-origin" -X POST -d "title=ok&body=y" "$BASE/notes" | head -1

hr "PUT /notes (wrong method) -> 405"
curl -s -i -H "$COOKIE" -X PUT "$BASE/notes" | head -3

hr "POST /notes/1/delete -> 303"
curl -s -i -H "$COOKIE" -X POST "$BASE/notes/1/delete" | head -3

hr "POST /logout -> 303 + cookie removal"
curl -s -i -H "$COOKIE" -X POST "$BASE/logout" | head -8

hr "after logout the old token no longer works"
curl -s -i -H "$COOKIE" "$BASE/notes" | head -1

kill $SERVER_PID 2>/dev/null
taskkill //F //IM topcoat-probe.exe > /dev/null 2>&1
rm -f "$JAR"
hr "server log"
cat smoke-server.log

#!/usr/bin/env bash
# 第11巻 UNIT 09 のエラーの例: サーバーが動いていない・curl の jar が Secure のクッキーを送らない。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
export VOL=rust11
CAP="bash $S/capx.sh"
O="$S/rust11/out"
URL=http://127.0.0.1:3013
cd "$S/rust11/rustnote" || exit 1

# エラー2: サーバーを起動し忘れた
taskkill //F //IM rustnote.exe > /dev/null 2>&1
$CAP u09_err_refused oha --no-tui -n 20 -c 2 "$URL/login" > /dev/null

# エラー3: curl のクッキーの入れ物（jar）で、ログインした人のページを試した
db="$S/rust11/u09_jar.db"; rm -f "$db"*
( HOST=127.0.0.1 PORT=3013 DATABASE_URL="$(cygpath -w "$db")" "$S/target11/release/rustnote.exe" > /dev/null 2> /dev/null & )
for i in $(seq 1 60); do curl.exe -s -o /dev/null "$URL/health" && break; sleep 0.5; done
jar="$S/rust11/u09_jar.txt"; rm -f "$jar"
{
  echo '$ curl.exe -s -o NUL -w "%{http_code}" -c jar.txt -X POST --data "email=…&password=…&password_confirm=…" http://127.0.0.1:3013/signup'
  curl.exe -s -o /dev/null -w "%{http_code}\n" -c "$jar" -X POST -H "Content-Type: application/x-www-form-urlencoded" \
    --data "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$URL/signup"
  echo '$ curl.exe -s -o NUL -w "%{http_code}" -b jar.txt http://127.0.0.1:3013/notes'
  curl.exe -s -o /dev/null -w "%{http_code}\n" -b "$jar" "$URL/notes"
  echo '$ （jar.txt の中の __Host-session の行）'
  grep "__Host-session" "$jar" | awk -F'\t' '{print "domain=" $1 " secure=" $4 " name=" $6 " value=" substr($7,1,6) "…"}'
} > "$O/u09_err_jar.txt" 2>&1
taskkill //F //IM rustnote.exe > /dev/null 2>&1
sleep 1
rm -f "$db"* "$jar"

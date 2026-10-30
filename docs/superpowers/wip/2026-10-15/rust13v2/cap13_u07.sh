#!/usr/bin/env bash
# 第13巻 UNIT 07 の出力を取る（画面の枠・部品・登録とログインの画面）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && diff -rq tests "$REAL/tests" > /dev/null && echo "src・migrations・tests は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
export PYTHONIOENCODING=utf-8
topcoat asset bundle --release > /dev/null 2>&1 || { echo "release の束を作れない"; exit 1; }
EXE="$H/target/release/rustnote.exe"

# ---- エラー1〜3
for e in e07_text e07_prop e07_href; do
  cp "$H/errs_u07/$e.rs" examples/
  $CAP "u07_err_${e#e07_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done
# ---- HTTP のテストのうち、この単元の分
$CAP u07_http_tests cargo test --test http -- health top_page signup login_and cross_site flash > /dev/null

# ---- 本物のサーバー（release）
RUN="$H/u07run"
B=http://127.0.0.1:3014
start() {
  rm -rf "$RUN"; mkdir -p "$RUN"
  ( cd "$RUN" && PORT=3014 DATABASE_URL=u07.db "$EXE" > "$RUN/server.log" 2>&1 & )
  for i in $(seq 1 40); do
    sleep 0.25
    curl.exe -s -o "$RUN/discard" "$B/health" && return 0
  done
  echo "起動しない"; cat "$RUN/server.log"; exit 1
}
stop() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
# req <名前> <見せる所> <curl の引数…>
req() { local name=$1 mode=$2; shift 2; curl.exe -s -i "$@" > "$RUN/$name.raw"; python "$H/http13.py" "$RUN/$name.raw" "$mode" | tr -d '\r' > "$O/$name.txt"; }
ck() { python "$H/cookie13.py" "$RUN/$1.raw" | tr -d '\r'; }
SIGNUP="email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery"

start
req u07_home html "$B/"
req u07_404 main "$B/nope"
req u07_400 main -d "email=a" "$B/login"
req u07_signup_ok none -d "$SIGNUP" "$B/signup"
C1=$(ck u07_signup_ok)
req u07_notes_header header -H "Cookie: $C1" "$B/notes"
req u07_signup_dup main -d "$SIGNUP" "$B/signup"
req u07_login_fail main -d "email=alice%40example.com&password=wrong-password" "$B/login"
req u07_health raw "$B/health"
# 問1: 3つともまちがえた登録
req u07_q1 main -d "email=alice&password=short&password_confirm=different" "$B/signup"
# 問2: GET でのログアウトと、ほかのサイトからの POST
req u07_q2_get none "$B/logout"
req u07_q2_origin raw -X POST -H "Origin: https://evil.example" -H "Cookie: $C1" "$B/logout"
req u07_q2_fetch raw -X POST -H "Sec-Fetch-Site: cross-site" -H "Cookie: $C1" "$B/logout"
req u07_q2_same none -X POST -H "Origin: $B" -H "Cookie: $C1" "$B/logout"
# 問3: タグを含むメールで登録して、ヘッダーを見る
req u07_q3_signup none -d "email=%3Cb%3Ebold%3C%2Fb%3E%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$B/signup"
C3=$(ck u07_q3_signup)
req u07_q3_header header -H "Cookie: $C3" "$B/notes"
grep -v "^$" "$RUN/server.log" | tr -d '\r' > "$O/u07_server_log.txt"
stop

check

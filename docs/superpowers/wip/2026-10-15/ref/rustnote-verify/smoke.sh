#!/usr/bin/env bash
# RustNote のスモークテスト: 実際にサーバーを起動して curl で一通り操作する。
# 使い方（Git Bash）: bash smoke.sh <rustnote.exe のパス> <DB ファイルのパス> <ポート>
set -u
EXE="$1"
DBFILE="$2"
PORT="${3:-3300}"
BASE="http://127.0.0.1:$PORT"
WORK=$(mktemp -d)
ALICE="$WORK/alice.jar"
BOB="$WORK/bob.jar"

rm -f "$DBFILE" "$DBFILE-wal" "$DBFILE-shm"
DATABASE_URL="$DBFILE" HOST=127.0.0.1 PORT=$PORT "$EXE" > "$WORK/server.log" 2>&1 &
for i in $(seq 1 50); do curl -s -o /dev/null "$BASE/health" && break; sleep 0.2; done

hr() { echo; echo "===== $*"; }
# 日本語を UTF-8 でパーセントエンコードする（Windows の curl.exe に日本語の引数を
# そのまま渡すと ANSI コードページ（CP932）に変換されて文字化けするため）
u() { python -c "import sys, urllib.parse; print(urllib.parse.quote(sys.argv[1], safe=''))" "$1"; }
# ステータス行と Location / Set-Cookie だけを見せる
head_of() { sed -n '1p;/^location:/Ip;/^set-cookie:/Ip'; }

hr "GET /health"
curl -s -i "$BASE/health" | sed -n '1p;$p'; echo

hr "GET / (未ログイン: 案内ページ) と CSS アセット"
HTML=$(curl -s "$BASE/")
echo "$HTML" | grep -o '<link rel="stylesheet" href="[^"]*">'
CSS=$(echo "$HTML" | grep -o '/_topcoat/assets/style-[^"]*\.css')
curl -s -i "$BASE$CSS" | sed -n '1p;/^content-type/Ip;/^cache-control/Ip'

hr "GET /notes 未ログイン -> 303 /login"
curl -s -i "$BASE/notes" | head_of

hr "POST /signup 不正な入力 -> 400"
curl -s -i -X POST -d "email=bad&password=short&password_confirm=short" "$BASE/signup" | sed -n '1p'
curl -s -X POST -d "email=bad&password=short&password_confirm=short" "$BASE/signup" | grep -o '<p class="field-error">[^<]*</p>'

hr "POST /signup alice -> 303 /notes + セッションのクッキー"
curl -s -i -c "$ALICE" -b "$ALICE" -X POST --data-urlencode "email=alice@example.com" --data-urlencode "password=correct horse battery" --data-urlencode "password_confirm=correct horse battery" "$BASE/signup" | head_of

hr "GET /notes（フラッシュ『登録しました』）"
curl -s -c "$ALICE" -b "$ALICE" "$BASE/notes" | grep -o '<p class="flash[^"]*"[^>]*>[^<]*</p>\|<h1>[^<]*</h1>\|<p class="empty">[^<]*</p>'

hr "POST /signup 同じメール -> 409"
curl -s -i -X POST --data-urlencode "email=ALICE@example.com" --data-urlencode "password=another password" --data-urlencode "password_confirm=another password" "$BASE/signup" | sed -n '1p'

hr "POST /logout -> 303 /login、ログイン画面にフラッシュ"
curl -s -i -c "$ALICE" -b "$ALICE" -X POST "$BASE/logout" | head_of
curl -s -c "$ALICE" -b "$ALICE" "$BASE/login" | grep -o '<p class="flash[^"]*"[^>]*>[^<]*</p>'

hr "POST /login パスワード違い -> 401"
curl -s -i -X POST --data-urlencode "email=alice@example.com" --data-urlencode "password=wrong password" "$BASE/login" | sed -n '1p'

hr "POST /login 正しい -> 303 /notes"
curl -s -i -c "$ALICE" -b "$ALICE" -X POST --data-urlencode "email=alice@example.com" --data-urlencode "password=correct horse battery" "$BASE/login" | head_of

hr "POST /notes 空タイトル -> 400"
curl -s -i -b "$ALICE" -X POST -d "title=$(u '   ')&body=$(u '本文')" "$BASE/notes" | sed -n '1p'
curl -s -b "$ALICE" -X POST -d "title=$(u '   ')&body=$(u '本文')" "$BASE/notes" | grep -o '<p class="field-error">[^<]*</p>'

hr "POST /notes 作成 -> 303 /notes/1"
curl -s -i -c "$ALICE" -b "$ALICE" -X POST -d "title=$(u '買い物 <b>リスト</b>')&body=$(u '牛乳 <script>alert(1)</script>')" "$BASE/notes" | head_of

hr "GET /notes/1（エスケープ・フラッシュ）"
curl -s -c "$ALICE" -b "$ALICE" "$BASE/notes/1" | grep -o '<p class="flash[^"]*"[^>]*>[^<]*</p>\|<h1>[^<]*</h1>\|<div class="note-body">[^<]*</div>'

hr "もう2件作成して一覧と検索"
curl -s -o /dev/null -b "$ALICE" -X POST -d "title=$(u 'Rust の勉強')&body=$(u '所有権と借用')" "$BASE/notes"
curl -s -o /dev/null -b "$ALICE" -X POST -d "title=$(u '100% やる')&body=" "$BASE/notes"
curl -s -b "$ALICE" "$BASE/notes" | grep -o '<a href="/notes/[0-9]*">[^<]*</a>\|<span>[^<]*ページ[^<]*</span>'
echo "--- ?q=所有権"
curl -s -b "$ALICE" "$BASE/notes?q=$(u '所有権')" | grep -o '<a href="/notes/[0-9]*">[^<]*</a>\|<span>[^<]*ページ[^<]*</span>'
echo "--- ?q=100%（% は普通の文字として検索）"
curl -s -b "$ALICE" "$BASE/notes?q=$(u '100%')" | grep -o '<a href="/notes/[0-9]*">[^<]*</a>'
echo "--- ?page=abc -> 400"
curl -s -o /dev/null -w "%{http_code}\n" -b "$ALICE" "$BASE/notes?page=abc"

hr "GET /notes/1/edit（version=1 が埋め込まれている）"
curl -s -b "$ALICE" "$BASE/notes/1/edit" | grep -o '<input type="hidden" name="version" value="[0-9]*">'

hr "POST /notes/1/edit version=1 -> 303"
curl -s -i -b "$ALICE" -X POST -d "title=$(u '買い物（更新）')&body=$(u '卵')&version=1" "$BASE/notes/1/edit" | head_of

hr "POST /notes/1/edit 古い version=1 -> 409（最新の version=2 で再表示）"
curl -s -i -b "$ALICE" -X POST -d "title=$(u '古い画面')&body=&version=1" "$BASE/notes/1/edit" | sed -n '1p'
curl -s -b "$ALICE" -X POST -d "title=$(u '古い画面')&body=&version=1" "$BASE/notes/1/edit" | grep -o '<p class="form-notice">[^<]*</p>\|<input type="hidden" name="version" value="[0-9]*">'

hr "POST /notes/1/toggle -> 303 /notes/1、完了バッジ"
curl -s -i -b "$ALICE" -X POST "$BASE/notes/1/toggle" | head_of
curl -s -b "$ALICE" "$BASE/notes/1" | grep -o '<span class="badge">[^<]*</span>' | head -1

hr "bob を登録。alice のノートは bob から 404"
curl -s -o /dev/null -c "$BOB" -b "$BOB" -X POST --data-urlencode "email=bob@example.com" --data-urlencode "password=bob password 1" --data-urlencode "password_confirm=bob password 1" "$BASE/signup"
for path in /notes/1 /notes/1/edit; do
  printf "GET  %-18s -> " "$path"; curl -s -o /dev/null -w "%{http_code}\n" -b "$BOB" "$BASE$path"
done
for path in /notes/1/edit /notes/1/toggle /notes/1/delete; do
  printf "POST %-18s -> " "$path"; curl -s -o /dev/null -w "%{http_code}\n" -b "$BOB" -X POST --data-urlencode "title=x" --data-urlencode "body=" --data-urlencode "version=1" "$BASE$path"
done
echo "--- 404 の画面"
curl -s -b "$BOB" "$BASE/notes/1" | grep -o '<h1>[^<]*</h1>\|<p>ページが見つかりません</p>'

hr "CSRF: 他サイトからの POST（Sec-Fetch-Site: cross-site）-> 403"
curl -s -i -b "$ALICE" -H "Sec-Fetch-Site: cross-site" -X POST --data-urlencode "title=attack" --data-urlencode "body=" "$BASE/notes" | sed -n '1p'
echo "--- Origin が違う（Sec-Fetch-Site なし）-> 403"
curl -s -i -b "$ALICE" -H "Origin: https://evil.example" -X POST "$BASE/notes/1/delete" | sed -n '1p'

hr "POST /notes/1/delete -> 303 /notes、その後 404"
curl -s -i -b "$ALICE" -X POST "$BASE/notes/1/delete" | head_of
curl -s -o /dev/null -w "GET /notes/1 -> %{http_code}\n" -b "$ALICE" "$BASE/notes/1"

hr "存在しない URL・不正な ID -> 404"
for path in /nope /notes/abc /notes/0; do
  printf "%-12s -> " "$path"; curl -s -o /dev/null -w "%{http_code}\n" -b "$ALICE" "$BASE$path"
done

hr "POST /logout -> 303、古いセッションでは /notes に入れない"
curl -s -i -c "$ALICE" -b "$ALICE" -X POST "$BASE/logout" | head_of
curl -s -o /dev/null -w "GET /notes -> %{http_code}\n" -b "$ALICE" "$BASE/notes"

hr "DB の中身（sessions / schema_migrations）"
echo "(サーバーのログを参照)"

taskkill //F //IM "$(basename "$EXE")" > /dev/null 2>&1
sleep 0.5
hr "サーバーのログ（標準エラー出力）"
cat "$WORK/server.log"
rm -rf "$WORK"

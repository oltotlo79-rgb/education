#!/usr/bin/env bash
# 第13巻 UNIT 08 の出力を取る（ノートの CRUD）。
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
bundle() { topcoat asset bundle --release > /dev/null 2>&1 || { echo "release の束を作れない"; exit 1; }; }
bundle
EXE="$H/target/release/rustnote.exe"

# ---- エラー2・3（コンパイルエラー）
for e in e08_path e08_form; do
  cp "$H/errs_u08/$e.rs" examples/
  $CAP "u08_err_${e#e08_}" cargo run -q --example "$e" > /dev/null
  rm "examples/$e.rs"
done
# ---- HTTP のテストのうち、この単元の分
$CAP u08_http_tests cargo test --test http -- notes_require create_note empty_title other_users invalid_ids edit_with toggle_and list_searches > /dev/null

RUN="$H/u08run"
B=http://127.0.0.1:3014
start() {
  rm -rf "$RUN"; mkdir -p "$RUN"
  ( cd "$RUN" && PORT=3014 DATABASE_URL=u08.db "$EXE" > "$RUN/server.log" 2>&1 & )
  for i in $(seq 1 40); do
    sleep 0.25
    curl.exe -s -o "$RUN/discard" "$B/health" && return 0
  done
  echo "起動しない"; cat "$RUN/server.log"; exit 1
}
stop() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
req() { local name=$1 mode=$2; shift 2; curl.exe -s -i "$@" > "$RUN/$name.raw"; python "$H/http13.py" "$RUN/$name.raw" "$mode" | tr -d '\r' > "$O/$name.txt"; }
ck() { python "$H/cookie13.py" "$RUN/$1.raw" | tr -d '\r'; }
signup() { req "$1" none -d "email=$2%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$B/signup"; }

start
signup u08_signup_alice alice
CA=$(ck u08_signup_alice)
req u08_create_ok none -H "Cookie: $CA" -d "title=%E8%B2%B7%E3%81%84%E7%89%A9&body=%E7%89%9B%E4%B9%B3%0D%0A%E5%8D%B5" "$B/notes"
req u08_show main -H "Cookie: $CA" "$B/notes/1"
{ echo "このときの PC の時計（日本時間）: $(date '+%Y-%m-%d %H:%M:%S')"; echo "このときの UTC: $(date -u '+%Y-%m-%d %H:%M:%S')"; } > "$O/u08_clock.txt"
python -c "import sys; open(sys.argv[1], 'w').write('title=&body=' + 'x' * 10001)" "$RUN/bad_form.txt"
req u08_create_bad main -H "Cookie: $CA" --data-binary "@$RUN/bad_form.txt" "$B/notes"
python "$H/longx13.py" "$O/u08_create_bad.txt"
req u08_list main -H "Cookie: $CA" "$B/notes"
req u08_page_abc main -H "Cookie: $CA" "$B/notes?page=abc"
req u08_page_99 main -H "Cookie: $CA" "$B/notes?page=99"
req u08_edit main -H "Cookie: $CA" "$B/notes/1/edit"
# 2つのタブ: どちらも version 1 の編集画面から保存する
req u08_save_a none -H "Cookie: $CA" -d "title=%E8%B2%B7%E3%81%84%E7%89%A9%EF%BC%88%E5%9C%9F%E6%9B%9C%EF%BC%89&body=%E7%89%9B%E4%B9%B3&version=1" "$B/notes/1/edit"
req u08_save_b main -H "Cookie: $CA" -d "title=%E8%B2%B7%E3%81%84%E7%89%A9%EF%BC%88%E6%97%A5%E6%9B%9C%EF%BC%89&body=%E5%8D%B5&version=1" "$B/notes/1/edit"
req u08_save_b2 none -H "Cookie: $CA" -d "title=%E8%B2%B7%E3%81%84%E7%89%A9%EF%BC%88%E6%97%A5%E6%9B%9C%EF%BC%89&body=%E5%8D%B5&version=2" "$B/notes/1/edit"
req u08_toggle none -H "Cookie: $CA" -X POST "$B/notes/1/toggle"
req u08_show_done main -H "Cookie: $CA" "$B/notes/1"
# 他人と、おかしな ID
signup u08_signup_bob bob
CB=$(ck u08_signup_bob)
req u08_bob_show main -H "Cookie: $CB" "$B/notes/1"
req u08_bob_toggle none -H "Cookie: $CB" -X POST "$B/notes/1/toggle"
req u08_bob_edit none -H "Cookie: $CB" -d "title=x&body=&version=4" "$B/notes/1/edit"
req u08_abc none -H "Cookie: $CA" "$B/notes/abc"
req u08_zero none -H "Cookie: $CA" "$B/notes/0"
req u08_missing none -H "Cookie: $CA" "$B/notes/999"
# NUL の題（第11巻 UNIT 04）
req u08_nul main -H "Cookie: $CA" -d "title=%00&body=" "$B/notes"
# 削除
req u08_delete none -H "Cookie: $CA" -X POST "$B/notes/1/delete"
req u08_after_delete none -H "Cookie: $CA" "$B/notes/1"
grep -v "^$" "$RUN/server.log" | python "$H/mask13.py" | tr -d '\r' > "$O/u08_server_log.txt"
stop

edit() {
  python - "$@" <<'PY'
import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
assert s.count(old) == 1, old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
}

# ---- エラー1: 一覧のリンクの href! に、ID を渡し忘れる（コンパイルは通る）
edit src/routes/notes.rs '<a href=(href!(id::show, id::Id(note.id)))>(note.title)</a>' '<a href=(href!(id::show))>(note.title)</a>'
$CAP u08_err1_build cargo build -q > /dev/null
bundle
start
signup u08_err1_signup alice
CA=$(ck u08_err1_signup)
req u08_err1_empty main -H "Cookie: $CA" "$B/notes"
req u08_err1_create none -H "Cookie: $CA" -d "title=a&body=" "$B/notes"
curl.exe -s -i -H "Cookie: $CA" "$B/notes" > "$RUN/u08_err1_list.raw"; echo "curl の終了コード: $?" > "$O/u08_err1_list.txt"
python "$H/http13.py" "$RUN/u08_err1_list.raw" none 2>&1 | tr -d '\r' >> "$O/u08_err1_list.txt"
grep -v "^$" "$RUN/server.log" | python "$H/mask13.py" | tr -d '\r' > "$O/u08_err1_log.txt"
stop
$CAP u08_err1_test cargo test -q --test http list_searches_and_paginates > /dev/null
cp "$REAL/src/routes/notes.rs" src/routes/notes.rs

# ---- 問1: ページの番号が大きすぎるときの文を直す
edit src/routes/notes.rs '                if keyword.is_some() {
                    "条件に合うノートはありません。"
                } else {' '                if page.total_count > 0 {
                    "このページにはノートがありません。"
                } else if keyword.is_some() {
                    "条件に合うノートはありません。"
                } else {'
bundle
start
signup u08_q1_signup alice
CA=$(ck u08_q1_signup)
req u08_q1_create none -H "Cookie: $CA" -d "title=a&body=" "$B/notes"
req u08_q1_page_99 main -H "Cookie: $CA" "$B/notes?page=99"
req u08_q1_page_1 main -H "Cookie: $CA" "$B/notes?q=zzz"
stop
$CAP u08_q1_test cargo test -q --test http > /dev/null
cp "$REAL/src/routes/notes.rs" src/routes/notes.rs

bundle
check

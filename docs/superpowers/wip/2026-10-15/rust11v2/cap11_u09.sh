#!/usr/bin/env bash
# 第11巻 UNIT 09 の出力を取る（release の RustNote に、oha で負荷をかける）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11"
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
O="$S/rust11/out"
URL=http://127.0.0.1:3013
LOGIN_FORM="email=alice%40example.com&password=correct+horse+battery"
cd "$S/rust11/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check

# サーバーを起動する（新しい DB。アクセスログは捨てずにファイルへ）
start_server() {
  local name=$1
  local db="$S/rust11/u09_$name.db"
  rm -f "$db"*
  ( HOST=127.0.0.1 PORT=3013 DATABASE_URL="$(cygpath -w "$db")" "$S/target11/release/rustnote.exe" > /dev/null 2> "$O/u09_${name}_server.log" & )
  for i in $(seq 1 60); do curl.exe -s -o /dev/null "$URL/health" && return 0; sleep 0.5; done
  echo "起動しない"; exit 1
}
stop_server() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; rm -f "$S"/rust11/u09_*.db*; }
# サーバーの、これまでのメモリのいちばん多い所（MiB）
peak_mb() { powershell -NoProfile -c "[int]((Get-Process rustnote).PeakWorkingSet64/1MB)" | tr -d '\r'; }
oha_run() { local name=$1; shift; $CAP "$name" oha --no-tui "$@" > /dev/null; }

cargo build --release -q || exit 1
topcoat asset bundle --release > /dev/null
echo "CPU: $NUMBER_OF_PROCESSORS 論理プロセッサ" > "$O/u09_machine.txt"
oha --version >> "$O/u09_machine.txt"

# ---- 例9-1: ふつうのページ
start_server plain
# alice を登録し、セッションのクッキーを取る（Secure のクッキーなので、curl の jar ではなく、ヘッダーから取り出す）
COOKIE=$(curl.exe -s -i -X POST -H "Content-Type: application/x-www-form-urlencoded" \
  --data "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$URL/signup" \
  | tr -d '\r' | grep -i "^set-cookie: __Host-session=" | sed 's/^[Ss]et-[Cc]ookie: //; s/;.*//')
for i in $(seq 1 20); do
  curl.exe -s -o /dev/null -X POST -H "Cookie: $COOKIE" -H "Content-Type: application/x-www-form-urlencoded" --data "title=memo$i&body=" "$URL/notes"
done
echo "start: $(peak_mb) MiB" > "$O/u09_peak_plain.txt"
oha_run u09_get_login -n 5000 -c 10 "$URL/login"
oha_run u09_get_notes -n 5000 -c 10 -H "Cookie: $COOKIE" "$URL/notes"
echo "after GET: $(peak_mb) MiB" >> "$O/u09_peak_plain.txt"

# ---- 例9-2: ログイン（Argon2）を、同時の数を変えて
for c in 1 4 12 48; do
  n=$((c * 10)); [ $n -lt 60 ] && n=60
  oha_run "u09_login_c$c" -n "$n" -c "$c" -m POST -T application/x-www-form-urlencoded -d "$LOGIN_FORM" "$URL/login"
  echo "c=$c: $(peak_mb) MiB" >> "$O/u09_peak_plain.txt"
done

# ---- エラー: 全部が断られているのに、速く見える（ほかのサイトからの POST の形で送った）
oha_run u09_err_403 -n 2000 -c 10 -m POST -T application/x-www-form-urlencoded -H "Sec-Fetch-Site: cross-site" -d "$LOGIN_FORM" "$URL/login"
grep -c "POST /login" "$O/u09_plain_server.log" > "$O/u09_plain_logcount.txt"
stop_server

# ---- 例9-3: 背圧（Argon2 を同時に計算する数を、CPU の数までにする）
$E src/auth.rs "$X/e09_slots.py" > /dev/null || exit 1
diff -u --label a/src/auth.rs --label b/src/auth.rs "$REAL/src/auth.rs" src/auth.rs > "$O/u09_diff_auth.txt"
cargo build --release -q || exit 1
start_server slots
curl.exe -s -o /dev/null -X POST -H "Content-Type: application/x-www-form-urlencoded" \
  --data "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$URL/signup"
echo "start: $(peak_mb) MiB" > "$O/u09_peak_slots.txt"
for c in 12 48; do
  n=$((c * 10))
  oha_run "u09_slots_c$c" -n "$n" -c "$c" -m POST -T application/x-www-form-urlencoded -d "$LOGIN_FORM" "$URL/login"
  echo "c=$c: $(peak_mb) MiB" >> "$O/u09_peak_slots.txt"
done
stop_server
cp "$REAL/src/auth.rs" src/auth.rs
cargo build --release -q
check

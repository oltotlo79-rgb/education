#!/usr/bin/env bash
# 第12巻 UNIT 10 の出力を取る（backup・復旧・更新）。手元の release の RustNote と、ローカルの DB ファイルで試す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
CAP="bash $H/cap.sh"
O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
EXE="$H/target/release/rustnote.exe"
COUNT="$H/target/debug/examples/u10_count.exe"
BACKUP="$H/target/debug/examples/u10_backup.exe"
B=http://127.0.0.1:3000
stop_all() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
start() { ( cd "$R" && DATABASE_URL=data/rustnote.db "$EXE" > /dev/null 2>> "$R/server.log" & ); for i in $(seq 1 40); do curl.exe -s -o /dev/null "$B/health" && return 0; sleep 0.25; done; }
note() { curl.exe -s -o /dev/null -b "$R/jar" -c "$R/jar" -d "title=$1&body=" "$B/notes"; }
files() { ls -l "$R/data" | awk 'NR>1 {print $NF ": " $5 " バイト"}'; }
cd "$H/rustnote" || exit 1
export CARGO_TARGET_DIR="$H/target"
cargo build -q --examples || exit 1
stop_all
R="$H/u10run"; rm -rf "$R"; mkdir -p "$R/data"

# ---- 例10-1: 動いている RustNote に、ユーザー1人とノート5件を作る
start
curl.exe -s -o /dev/null -c "$R/jar" -d "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$B/signup"
for k in 1 2 3 4 5; do note "memo$k"; done
{
  echo "--- 動いている間の data フォルダ"
  files
} > "$O/u10_files_running.txt"

# ---- 例10-2: 動いている間に、2通りで写し取る
cp "$R/data/rustnote.db" "$R/naive.db"
( cd "$R" && "$BACKUP" data/rustnote.db backup.db ) > "$R/backup.out" 2>&1
{
  echo "--- VACUUM INTO で写す（u10_backup）"
  tr -d '\r' < "$R/backup.out"
  echo "--- rustnote.db だけをコピーしたもの（naive.db）"
  ( cd "$R" && "$COUNT" naive.db ) 2>&1 | tr -d '\r'
  echo "--- VACUUM INTO で写したもの（backup.db）"
  ( cd "$R" && "$COUNT" backup.db ) 2>&1 | tr -d '\r'
} > "$O/u10_backup.txt"

# ---- 例10-3: バックアップのあとに、もう3件作ってから、バックアップに戻す
for k in 6 7 8; do note "memo$k"; done
stop_all
{
  echo "--- 戻す前（止めたあとの data/rustnote.db）"
  ( cd "$R" && "$COUNT" data/rustnote.db ) 2>&1 | tr -d '\r'
} > "$O/u10_restore.txt"
mkdir -p "$R/broken"; mv "$R"/data/rustnote.db* "$R/broken/"
cp "$R/backup.db" "$R/data/rustnote.db"
start
LOGIN=$(curl.exe -s -o /dev/null -w '%{http_code}' -c "$R/jar2" -d "email=alice%40example.com&password=correct+horse+battery" "$B/login")
LIST=$(curl.exe -s -b "$R/jar2" "$B/notes" | grep -o '全[0-9]*件')
{
  echo "--- バックアップに戻して起動"
  echo "ログイン: $LOGIN・一覧: $LIST"
} >> "$O/u10_restore.txt"
stop_all
mask < "$R/server.log" | grep -v "^$" > "$O/u10_server_log.txt"
rm -rf "$R"

# ---- 例10-4: 依存の更新を確かめる（Cargo.lock は書き換えない）
$CAP u10_update_dry cargo update --dry-run > /dev/null
$CAP u10_audit cargo audit > /dev/null
echo done

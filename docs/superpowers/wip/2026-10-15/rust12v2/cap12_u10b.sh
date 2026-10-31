#!/usr/bin/env bash
# 第12巻 UNIT 10: バックアップの取り方（だれも開いていないとき・ほかの接続が開いているとき）と、戻し方（正しい・間違い）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
EXE="$H/target/release/rustnote.exe"
EX="$H/target/debug/examples"
B=http://127.0.0.1:3000
stop_all() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; taskkill //F //IM u10_hold.exe > /dev/null 2>&1; sleep 1; }
start() {  # フォルダ: そのフォルダの data/rustnote.db で起動する
  ( cd "$1" && DATABASE_URL=data/rustnote.db "$EXE" > /dev/null 2>> "$1/server.log" & )
  for i in $(seq 1 40); do curl.exe -s -o /dev/null "$B/health" && return 0; sleep 0.25; done
}
note() { curl.exe -s -o /dev/null -b "$R/jar" -c "$R/jar" -d "title=$1&body=" "$B/notes"; }
files() { ls -l "$1" | awk 'NR>1 && $1 !~ /^d/ {print "  " $NF ": " $5 " バイト"}'; }
count() { ( cd "$1" && "$EX/u10_count.exe" "$2" ) 2>&1 | tr -d '\r' | sed 's/^/  /'; }
login_list() {  # ログインして、一覧の「全N件」を読む
  local jar="$1/jar2" code list
  code=$(curl.exe -s -o /dev/null -w '%{http_code}' -c "$jar" -d "email=alice%40example.com&password=correct+horse+battery" "$B/login")
  list=$(curl.exe -s -o /dev/null -w '%{http_code}' -b "$jar" "$B/notes")
  echo "  ログイン: $code・一覧: $list $(curl.exe -s -b "$jar" "$B/notes" | grep -o '全[0-9]*件' | head -1)"
}
cd "$H/rustnote" || exit 1
CARGO_TARGET_DIR="$H/target" cargo build -q --examples || exit 1
stop_all
R="$H/u10b"; rm -rf "$R"; mkdir -p "$R/data"

start "$R"
curl.exe -s -o /dev/null -c "$R/jar" -d "email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery" "$B/signup"
for k in 1 2 3 4 5; do note "memo$k"; done

# ---- 例10-1 場面A: RustNote だけが動いていて、いまはだれもリクエストを送っていない
cp "$R/data/rustnote.db" "$R/copyA.db"
{
  echo "--- 場面A: data フォルダ（ノートを5件作ったあと）"
  files "$R/data"
  echo "--- rustnote.db だけをコピーしたもの（copyA.db）"
  count "$R" copyA.db
} > "$O/u10_backup_a.txt"

# ---- 例10-1 場面B: ほかの道具が、DB に接続を開いたままにしている
"$EX/u10_hold.exe" "$R/data/rustnote.db" 60 > "$R/hold.out" 2>&1 &
sleep 2
for k in 6 7 8; do note "memo$k"; done
cp "$R/data/rustnote.db" "$R/copyB.db"
( cd "$R" && "$EX/u10_backup.exe" data/rustnote.db backup.db ) > "$R/backup.out" 2>&1
{
  echo "--- 場面B: ほかの接続（u10_hold）"
  tr -d '\r' < "$R/hold.out" | sed 's/^/  /'
  echo "--- 場面B: data フォルダ（ノートをもう3件、全部で8件作ったあと）"
  files "$R/data"
  echo "--- rustnote.db だけをコピーしたもの（copyB.db）"
  count "$R" copyB.db
  echo "--- VACUUM INTO で写したもの（backup.db）"
  tr -d '\r' < "$R/backup.out" | sed 's/^/  /'
  count "$R" backup.db
} > "$O/u10_backup_b.txt"

# ---- 例10-2: バックアップのあとに2件作り、そこで突然止まった（マシンの故障の代わりに、強制終了）
for k in 9 10; do note "memo$k"; done
stop_all
mkdir -p "$R/crash"; cp "$R"/data/rustnote.db* "$R/crash/"
{
  echo "--- 止まったあとの data フォルダ"
  files "$R/data"
  echo "--- 止まったあとの DB（-wal も一緒にあるもの）を数える"
  count "$R" crash/rustnote.db
} > "$O/u10_crash.txt"

# 間違った戻し方: rustnote.db だけを backup.db で上書きして、-wal と -shm を残す
W="$R/wrong"; mkdir -p "$W/data"; cp "$R"/data/rustnote.db* "$W/data/"
cp "$R/backup.db" "$W/data/rustnote.db"
{
  echo "--- 間違った戻し方: rustnote.db だけを backup.db で上書き（-wal・-shm は残したまま）"
  files "$W/data"
  echo "--- 数える"
  count "$W" data/rustnote.db
} > "$O/u10_restore_wrong.txt"
W2="$R/wrong2"; mkdir -p "$W2/data"; cp "$R"/data/rustnote.db* "$W2/data/"
cp "$R/backup.db" "$W2/data/rustnote.db"
start "$W2"
{ echo "--- そのフォルダで RustNote を起動して、ログインと一覧"; login_list "$W2"; echo "--- RustNote のログ"; mask < "$W2/server.log" | tail -4 | sed 's/^/  /'; } >> "$O/u10_restore_wrong.txt"
stop_all

# 正しい戻し方: rustnote.db・-wal・-shm を全部よけてから、backup.db を置く
G="$R/right"; mkdir -p "$G/data" "$G/old"; cp "$R"/data/rustnote.db* "$G/data/"
mv "$G"/data/rustnote.db* "$G/old/"
cp "$R/backup.db" "$G/data/rustnote.db"
start "$G"
{
  echo "--- 正しい戻し方: 古い rustnote.db・-wal・-shm を old/ によけて、backup.db を rustnote.db として置く"
  files "$G/data"
  echo "--- そのフォルダで RustNote を起動して、ログインと一覧"
  login_list "$G"
} > "$O/u10_restore_right.txt"
stop_all
mask < "$R/server.log" > "$O/u10_server_log.txt"
echo done

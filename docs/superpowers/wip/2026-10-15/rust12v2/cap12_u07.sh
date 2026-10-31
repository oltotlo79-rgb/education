#!/usr/bin/env bash
# 第12巻 UNIT 07 の出力を取る（migration を出す順番と release command）。
# 手元の sqld（libSQL のサーバー）に、Windows の release の RustNote を何台かつなぐ。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
W="$H/u07"
T="$H/target-u07"
export PYTHONIOENCODING=utf-8
SQLD=ghcr.io/tursodatabase/libsql-server:latest
DB=http://127.0.0.1:18081
mask() { python "$H/mask12.py" | tr -d '\r'; }
stop_all() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
fresh_sqld() { docker rm -f sqld-u07 > /dev/null 2>&1; docker run -d --name sqld-u07 -p 18081:8080 "$SQLD" > /dev/null; sleep 3; }
edit() {
  python - "$@" <<'PY'
import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
assert s.count(old) == 1, old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
}
copy_src() {  # 名前
  local c="$W/$1"; mkdir -p "$c"
  cp -r "$REAL/src" "$REAL/migrations" "$REAL/assets" "$c/"
  cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$c/"
}
build_to() {  # 名前: release で作り、bin/名前/ に実行ファイルと assets を置く
  local c="$W/$1"
  # 同じ target を使い回すので、ソースの時刻を新しくして、cargo に作り直させる
  find "$c/src" "$c/migrations" -type f -exec touch {} +
  (cd "$c" && CARGO_TARGET_DIR="$T" cargo build -q --release --locked && CARGO_TARGET_DIR="$T" topcoat asset bundle --release > /dev/null) || { echo "build failed: $1"; exit 1; }
  mkdir -p "$W/bin/$1"
  cp "$T/release/rustnote.exe" "$W/bin/$1/"
  rm -rf "$W/bin/$1/assets"; cp -r "$T/release/assets" "$W/bin/$1/assets"
}
run_bg() {  # 名前・版・ポート
  local name=$1 ver=$2 port=$3
  ( PORT=$port DATABASE_URL=$DB "$W/bin/$ver/rustnote.exe" > /dev/null 2> "$W/$name.err" & )
}
wait_port() { for i in $(seq 1 60); do curl.exe -s -o /dev/null "http://127.0.0.1:$1/health" && return 0; sleep 0.25; done; return 1; }

stop_all
rm -rf "$W"; mkdir -p "$W"
docker pull -q "$SQLD" > /dev/null

# ---- 版を3つ作る
# v1: 教材のまま
copy_src v1
# v2: --migrate-only（マイグレーションだけして終わる）と、足すだけのマイグレーション 0004
copy_src v2
edit "$W/v2/src/main.rs" '    let config = Config::from_env().context("設定を読み込めませんでした")?;
' '    let config = Config::from_env().context("設定を読み込めませんでした")?;

    // `rustnote --migrate-only` は、マイグレーションだけをして終わる（Fly.io の release_command 用）
    if std::env::args().any(|arg| arg == "--migrate-only") {
        let db = Db::open(&config)
            .await
            .with_context(|| format!("DB（{}）を開けませんでした", config.database_url))?;
        let applied = db::migrate(&db)
            .await
            .context("マイグレーションに失敗しました")?;
        eprintln!("マイグレーションだけを行いました（新しく適用: {applied:?}）");
        return Ok(());
    }
'
printf -- '-- 0004: ノートに「ピン留め」の列を足す（足すだけ。古い版のコードは、この列を知らなくても動く）\nALTER TABLE notes ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0 CHECK (pinned IN (0, 1));\n' > "$W/v2/migrations/0004_pinned.sql"
edit "$W/v2/src/db.rs" '    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
];' '    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
    Migration {
        version: "0004_pinned",
        sql: include_str!("../migrations/0004_pinned.sql"),
    },
];'
# v3: 消してしまうマイグレーション 0004（本文の列の名前を変える）
copy_src v3
printf -- '-- 0004: 本文の列の名前を body から content に変える（古い版のコードは body を探して失敗する）\nALTER TABLE notes RENAME COLUMN body TO content;\n' > "$W/v3/migrations/0004_rename_body.sql"
edit "$W/v3/src/db.rs" '    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
];' '    Migration {
        version: "0003_sessions",
        sql: include_str!("../migrations/0003_sessions.sql"),
    },
    Migration {
        version: "0004_rename_body",
        sql: include_str!("../migrations/0004_rename_body.sql"),
    },
];'
for v in v1 v2 v3; do build_to $v; done
# 3つの実行ファイルが本当に違うかを確かめる
{
  for v in v1 v2 v3; do echo "$v の rustnote.exe の SHA-256（先頭 16 文字）: $(sha256sum "$W/bin/$v/rustnote.exe" | cut -c1-16)"; done
} > "$O/u07_bins.txt"
(cd "$W" && diff -u "$REAL/src/main.rs" v2/src/main.rs | sed 's#^--- .*#--- a/src/main.rs#; s#^+++ .*#+++ b/src/main.rs#') | tr -d '\r' > "$O/u07_diff_main.txt"

# ---- 例7-1: 空の DB に、3台の RustNote を同時に起動する（どれも起動のときに migrate する）
fresh_sqld
for k in 1 2 3; do run_bg race$k v1 1801$k; done
sleep 6
{
  for k in 1 2 3; do
    echo "--- 3台のうちの $k 台目（ポート 1801$k）:/health $(curl.exe -s -o /dev/null -w '%{http_code}' http://127.0.0.1:1801$k/health)"
    mask < "$W/race$k.err"
  done
} > "$O/u07_race.txt"
stop_all

# ---- 例7-2: release command の形: 先に --migrate-only を1回、そのあと3台を起動する
fresh_sqld
( DATABASE_URL=$DB "$W/bin/v2/rustnote.exe" --migrate-only > /dev/null 2> "$W/release.err" ); echo "（終了コード: $?）" >> "$W/release.err"
for k in 1 2 3; do run_bg after$k v2 1801$k; done
sleep 5
{
  echo "--- release command（rustnote --migrate-only）"
  mask < "$W/release.err"
  for k in 1 2 3; do
    echo "--- $k 台目（ポート 1801$k）: /health $(curl.exe -s -o /dev/null -w '%{http_code}' http://127.0.0.1:1801$k/health)"
    mask < "$W/after$k.err"
  done
} > "$O/u07_release_command.txt"
stop_all

# ---- 例7-3: 足すだけの 0004（v2）を当てたあとで、古い v1 を動かす
SIGNUP="email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery"
smoke() {  # ポート: 登録 → ノートを作る → 一覧
  local port=$1 jar="$W/jar$1"
  local s c l
  s=$(curl.exe -s -o /dev/null -w '%{http_code}' -c "$jar" -d "$SIGNUP" "http://127.0.0.1:$port/signup")
  if [ "$s" != "303" ]; then s="$s（もう登録済みならログイン: $(curl.exe -s -o /dev/null -w '%{http_code}' -c "$jar" -d 'email=alice%40example.com&password=correct+horse+battery' "http://127.0.0.1:$port/login")）"; fi
  c=$(curl.exe -s -o /dev/null -w '%{http_code}' -b "$jar" -d "title=v$2&body=memo" "http://127.0.0.1:$port/notes")
  l=$(curl.exe -s -o /dev/null -w '%{http_code}' -b "$jar" "http://127.0.0.1:$port/notes")
  echo "登録 $s・ノートの作成 $c・一覧 $l"
}
fresh_sqld
run_bg add2 v2 18021; wait_port 18021
{ echo "--- v2（0004_pinned を当てる）"; smoke 18021 2; } > "$O/u07_compat_add.txt"
stop_all
run_bg add1 v1 18022; wait_port 18022
{ echo "--- そのあと v1（教材のまま）を、同じ DB で動かす"; smoke 18022 1; mask < "$W/add1.err" | tail -3; } >> "$O/u07_compat_add.txt"
stop_all

# ---- 例7-4: 名前を変える 0004（v3）を当てたあとで、古い v1 を動かす
fresh_sqld
run_bg ren3 v3 18031; wait_port 18031
{ echo "--- v3（0004_rename_body を当てる）"; mask < "$W/ren3.err" | grep -E "マイグレーション"; } > "$O/u07_compat_rename.txt"
stop_all
run_bg ren1 v1 18032; wait_port 18032
{ echo "--- そのあと v1（教材のまま）を、同じ DB で動かす"; smoke 18032 1; echo "--- v1 のログ"; mask < "$W/ren1.err" | tail -4; } >> "$O/u07_compat_rename.txt"
stop_all

docker rm -f sqld-u07 > /dev/null 2>&1
echo done

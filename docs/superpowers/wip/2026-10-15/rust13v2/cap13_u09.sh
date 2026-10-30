#!/usr/bin/env bash
# 第13巻 UNIT 09 の出力を取る（見た目: assets/style.css）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
cd "$H/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq assets "$REAL/assets" > /dev/null && diff -rq tests "$REAL/tests" > /dev/null && echo "src・assets・tests は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
export CARGO_TARGET_DIR="$H/target"
export PYTHONIOENCODING=utf-8
bundle() { topcoat asset bundle --release > /dev/null 2>&1 || { echo "release の束を作れない"; exit 1; }; }
bundle
EXE="$H/target/release/rustnote.exe"

# ---- コントラスト比
python "$H/contrast13.py" assets/style.css | tr -d '\r' > "$O/u09_contrast.txt"

RUN="$H/u09run"
B=http://127.0.0.1:3014
start() {
  rm -rf "$RUN"; mkdir -p "$RUN"
  ( cd "$RUN" && PORT=3014 DATABASE_URL=u09.db "$EXE" > "$RUN/server.log" 2>&1 & )
  for i in $(seq 1 40); do
    sleep 0.25
    curl.exe -s -o "$RUN/discard" "$B/health" && return 0
  done
  echo "起動しない"; cat "$RUN/server.log"; exit 1
}
stop() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; sleep 1; }
css_href() { curl.exe -s "$B/" | grep -o '/_topcoat/assets/[^"]*'; }
measure() { (cd "$H/u09lab" && node measure.mjs "$1" 2>&1 | tr -d '\r'); }
edit() {
  python - "$@" <<'PY'
import sys
p, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
s = open(p, encoding="utf-8").read()
assert s.count(old) == 1, old
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new))
PY
}

# ---- 例9-2: CSS の配信と、本物の Chrome での計測
start
HREF=$(css_href)
{
  echo "HTML の <link> の href: $HREF"
  curl.exe -s -I "$B$HREF" | tr -d '\r' | grep -iE "^(HTTP|content-type|content-length|cache-control|etag)"
  echo "--- 中身の先頭 120 バイト"
  curl.exe -s "$B$HREF" | head -c 120; echo
  echo "--- 印（ハッシュ）を変えた URL"
  curl.exe -s -o "$RUN/discard" -w "%{http_code}\n" "$B/_topcoat/assets/style-0000000000000000.css"
} > "$O/u09_serve.txt"
measure all > "$O/u09_measure.txt"
stop

# ---- 問1: ヘッダーを折り返せるようにする
edit assets/style.css '  justify-content: space-between;
  gap: 16px;
  padding: 12px 24px;' '  justify-content: space-between;
  flex-wrap: wrap;
  gap: 16px;
  padding: 12px 24px;'
edit assets/style.css '.site-header nav {
  display: flex;
  align-items: center;' '.site-header nav {
  display: flex;
  flex-wrap: wrap;
  align-items: center;'
bundle
start
measure all | grep -E "^(--- 画面|.container|詳細)" > "$O/u09_q1_measure.txt"
stop
cp "$REAL/assets/style.css" assets/style.css

# ---- 問3: CSS を1文字変えると、URL はどうなるか
edit assets/style.css '/* RustNote のスタイル（素の CSS。Tailwind などは使わない） */' '/* RustNote のスタイル（素の CSS。Tailwind などは使わない）。問3の実験 */'
bundle
start
NEW=$(css_href)
{
  echo "変える前の href: $HREF"
  echo "変えたあとの href: $NEW"
  echo "変える前の URL を開くと: $(curl.exe -s -o "$RUN/discard" -w "%{http_code}" "$B$HREF")"
} > "$O/u09_q3.txt"
stop
cp "$REAL/assets/style.css" assets/style.css

# ---- エラー1: asset! のファイルの名前を書き間違える
edit src/views.rs 'asset!("assets/style.css")' 'asset!("assets/styles.css")'
$CAP u09_err1_build cargo build -q > /dev/null
$CAP u09_err1_bundle topcoat asset bundle --release > /dev/null
cp "$REAL/src/views.rs" src/views.rs

# ---- エラー2: 閉じ括弧を1つ書き忘れる
edit assets/style.css '.empty {
  color: var(--color-muted);
}' '.empty {
  color: var(--color-muted);
'
$CAP u09_err2_build cargo build -q > /dev/null
bundle
start
measure broken > "$O/u09_err2_measure.txt"
stop
cp "$REAL/assets/style.css" assets/style.css

# ---- エラー3: CSS の変数の名前を書き間違える
edit assets/style.css 'a {
  color: var(--color-primary);
}' 'a {
  color: var(--color-primay);
}'
bundle
start
measure typo > "$O/u09_err3_measure.txt"
stop
cp "$REAL/assets/style.css" assets/style.css

bundle
check

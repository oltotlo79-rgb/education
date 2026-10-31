#!/usr/bin/env bash
# 第12巻 UNIT 03 の出力を取る（アセットの束を本番用に作る）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
CAP="bash $H/cap.sh"
O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
stop_all() { taskkill //F //IM rustnote.exe > /dev/null 2>&1; docker stop rn-u03 > /dev/null 2>&1; sleep 1; }
stop_all
# 練習用のコピー（CSS を書き換えて試すので、別に作る）
W="$H/u03"; rm -rf "$W"; mkdir -p "$W"
cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$W/"
cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$W/"
T="$H/target-u03"
rm -rf "$T"
export WORKDIR=u03

# ---- 例3-1: 何も無い target で、topcoat asset bundle --release だけを動かす
# cap.sh は CARGO_TARGET_DIR を rust12/target にするので、ここは自分で動かす
( cd "$W" && CARGO_TARGET_DIR="$T" topcoat asset bundle --release ) > "$H/u03_bundle_only.raw" 2>&1
echo "exit=$?" >> "$H/u03_bundle_only.raw"
mask < "$H/u03_bundle_only.raw" | grep -vE "^\s*$" > "$O/u03_bundle_only.txt"
{
  echo "--- target-u03/release の中（実行ファイルと assets）"
  ls -l "$T/release" | grep -E "rustnote.exe$|assets$" | awk '{print $1, $5, $NF}'
  echo "--- assets の中"
  ls -l "$T/release/assets" | awk 'NR>1 {print $5, $NF}'
  echo "--- manifest.toml"
  cat "$T/release/assets/manifest.toml"
} | tr -d '\r' > "$O/u03_bundle_files.txt"

# ---- 例3-2: 配信のされ方（release の実行ファイルを起動して、CSS を取る）
RUN="$H/u03run"; rm -rf "$RUN"; mkdir -p "$RUN"
( cd "$RUN" && PORT=3014 "$T/release/rustnote.exe" > /dev/null 2> "$RUN/err" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3014/health && break; done
HREF=$(curl.exe -s http://127.0.0.1:3014/ | grep -o '/_topcoat/assets/[^"]*')
{
  echo "HTML の <link>: $HREF"
  for enc in "" "gzip" "br"; do
    if [ -z "$enc" ]; then echo "--- Accept-Encoding なし"; curl.exe -s -o "$RUN/h" -D - "http://127.0.0.1:3014$HREF" > "$RUN/h2"; else echo "--- Accept-Encoding: $enc"; curl.exe -s -o "$RUN/h" -D - -H "Accept-Encoding: $enc" "http://127.0.0.1:3014$HREF" > "$RUN/h2"; fi
    tr -d '\r' < "$RUN/h2" | grep -iE "^(HTTP|content-type|content-length|content-encoding|cache-control|etag|vary)"
    echo "受け取ったバイト数: $(stat -c %s "$RUN/h")"
  done
} > "$O/u03_serve.txt"
stop_all

# ---- 例3-3: CSS を変えて、束を作り直さずに起動する（古い束のまま）
printf '\n/* 第12巻 UNIT 03 の実験 */\n.hero { border-top: 4px solid var(--color-primary); }\n' >> "$W/assets/style.css"
( cd "$RUN" && PORT=3014 "$T/release/rustnote.exe" > /dev/null 2>> "$RUN/err" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3014/health && break; done
OLD=$(curl.exe -s http://127.0.0.1:3014/ | grep -o '/_topcoat/assets/[^"]*')
{
  echo "--- CSS を変えて、束を作り直さずに起動"
  echo "HTML の <link>: $OLD"
  echo "CSS の中に .hero の行があるか: $(curl.exe -s "http://127.0.0.1:3014$OLD" | grep -c 'border-top: 4px') 行"
} > "$O/u03_stale.txt"
stop_all
( cd "$W" && CARGO_TARGET_DIR="$T" topcoat asset bundle --release ) > /dev/null 2>&1
( cd "$RUN" && PORT=3014 "$T/release/rustnote.exe" > /dev/null 2>> "$RUN/err" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3014/health && break; done
NEW=$(curl.exe -s http://127.0.0.1:3014/ | grep -o '/_topcoat/assets/[^"]*')
{
  echo "--- topcoat asset bundle --release で作り直して起動"
  echo "HTML の <link>: $NEW"
  echo "CSS の中に .hero の行があるか: $(curl.exe -s "http://127.0.0.1:3014$NEW" | grep -c 'border-top: 4px') 行"
  echo "前の URL: $(curl.exe -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:3014$OLD")"
  echo "--- assets の中"
  ls -1 "$T/release/assets"
} >> "$O/u03_stale.txt"
stop_all
cp "$REAL/assets/style.css" "$W/assets/style.css"

# ---- エラー: 版の違う topcoat CLI（0.10.0）で、0.11.0 の RustNote の束を作る
( cd "$W" && CARGO_TARGET_DIR="$H/target-u03old" "$H/cli/bin/topcoat.exe" asset bundle --release ) > "$H/u03_oldcli.raw" 2>&1
echo "exit=$?" >> "$H/u03_oldcli.raw"
mask < "$H/u03_oldcli.raw" | grep -vE "^\s*$" | tail -15 > "$O/u03_err_oldcli.txt"
# 古い CLI で作った束でも、0.11.0 の実行ファイルは CSS を配れるか
( cd "$RUN" && PORT=3014 "$H/target-u03old/release/rustnote.exe" > /dev/null 2>> "$RUN/err" & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3014/health && break; done
OLDHREF=$(curl.exe -s http://127.0.0.1:3014/ | grep -o '/_topcoat/assets/[^"]*')
{
  echo "--- 古い CLI（0.10.0）で作った束で起動"
  echo "HTML の <link>: $OLDHREF"
  echo "その CSS: $(curl.exe -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:3014$OLDHREF")"
} >> "$O/u03_err_oldcli.txt"
stop_all
"$H/cli/bin/topcoat.exe" --version > "$O/u03_oldcli_version.txt" 2>&1

# ---- コンテナの中の束（UNIT 02 のイメージ）
docker run --rm rustnote:u02 sh -c 'ls -l /app/assets; cat /app/assets/manifest.toml' > "$O/u03_container_assets.txt" 2>&1
rm -rf "$RUN"
echo done

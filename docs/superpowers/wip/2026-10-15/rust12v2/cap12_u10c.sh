#!/usr/bin/env bash
# 第12巻 UNIT 10: 前の版のイメージに戻す（rollback）を、手元の Docker で試す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
export PYTHONIOENCODING=utf-8
P=http://127.0.0.1:18080
SMOKE="$(cygpath -w "$H/smoke.ps1")"
smoke() { powershell -NoProfile -ExecutionPolicy Bypass -File "$SMOKE" -Base "$1" 2>&1 | tr -d '\r'; echo "（終了コード: ${PIPESTATUS[0]}）"; }
wait_up() { for i in $(seq 1 50); do curl.exe -s -o /dev/null "$P/health" && return 0; sleep 0.2; done; }
run() { docker rm -f rn-u10 > /dev/null 2>&1; docker run -d --name rn-u10 -p 18080:8080 -v rn-u10data:/data "$1" > /dev/null; wait_up; }
W="$H/u10c"; rm -rf "$W"; mkdir -p "$W/v2"
( cd "$REAL" && cp -r src migrations assets tests Cargo.toml Cargo.lock rust-toolchain.toml Dockerfile .dockerignore "$W/v2/" )
python - "$W/v2/src/repository/notes.rs" <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
old = "ORDER BY updated_at DESC, id DESC"
assert s.count(old) == 1
open(p, "w", encoding="utf-8", newline="").write(s.replace(old, "ORDER BY update_at DESC, id DESC"))
PY
( cd "$W" && diff -u "$REAL/src/repository/notes.rs" v2/src/repository/notes.rs | sed 's#^--- .*#--- a/src/repository/notes.rs#; s#^+++ .*#+++ b/src/repository/notes.rs#' ) | tr -d '\r' > "$O/u10_v2_diff.txt"

docker tag rustnote:u02 rustnote:v1
start=$(date +%s)
docker build -q -t rustnote:v2 "$W/v2" > /dev/null || { echo "build failed"; exit 1; }
echo "v2 のイメージを作るのにかかった時間: $(( $(date +%s) - start )) 秒" > "$O/u10_v2_build.txt"
docker images rustnote --format '{{.Repository}}:{{.Tag}}  {{.ID}}  {{.Size}}' | grep -E ':v[12] ' > "$O/u10_images.txt"

docker volume rm rn-u10data > /dev/null 2>&1
# ---- v1 で公開している（データが入っている）
run rustnote:v1
smoke "$P" > "$O/u10_smoke_v1.txt"
# ---- v2 を公開した
run rustnote:v2
{
  echo "GET /health: $(curl.exe -s -o /dev/null -w '%{http_code}' "$P/health")"
  smoke "$P"
} > "$O/u10_smoke_v2.txt"
docker logs rn-u10 2>&1 | tr -d '\r' | grep -E "error|500" | head -3 > "$O/u10_v2_log.txt"
# ---- 前の版（v1）に戻す。DB は同じボリューム（rn-u10data）のまま
run rustnote:v1
smoke "$P" > "$O/u10_smoke_back.txt"
docker rm -f rn-u10 > /dev/null 2>&1
docker volume rm rn-u10data > /dev/null 2>&1

# ---- v2 のまちがいは、cargo test で見つかったか
cd "$W/v2" || exit 1
find src tests -type f -exec touch {} +
CARGO_TARGET_DIR="$H/target" cargo test -q 2>&1 | tr -d '\r' | grep -E "^test result|FAILED|panicked|failures:|^    [a-z_:]+$" | head -20 > "$O/u10_v2_test.txt"
echo done

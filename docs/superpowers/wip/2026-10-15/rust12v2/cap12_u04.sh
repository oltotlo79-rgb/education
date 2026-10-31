#!/usr/bin/env bash
# 第12巻 UNIT 04 の出力を取る（fly.toml とポート）。Fly.io にはつながず、fly.toml と Dockerfile を読み、手元のコンテナで確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
export PYTHONIOENCODING=utf-8
IMG=rustnote:u02
P=http://127.0.0.1:18080
CHECK="$H/rustnote/check_fly.py"
docker rm -f rn-u04 > /dev/null 2>&1
python --version > "$O/u04_python_version.txt" 2>&1

# ---- 例4-1: fly.toml を読んで、全体を表示する（TOML として読めるか）
python -c "import tomllib, json, sys; print(json.dumps(tomllib.load(open(sys.argv[1], 'rb')), ensure_ascii=False, indent=2))" "$REAL/fly.toml" | tr -d '\r' > "$O/u04_fly_json.txt"

# ---- 例4-2: 教材の fly.toml と Dockerfile を、動いているコンテナと合わせて確かめる
docker run -d --name rn-u04 -p 18080:8080 "$IMG" > /dev/null
for i in $(seq 1 50); do curl.exe -s -o /dev/null "$P/health" && break; sleep 0.2; done
python "$CHECK" "$REAL" "$P" 2>&1 | tr -d '\r' > "$O/u04_check_ok.txt"; echo "（終了コード: ${PIPESTATUS[0]}）" >> "$O/u04_check_ok.txt"
docker rm -f rn-u04 > /dev/null

# ---- 例4-3: よくある書き間違いを3つ入れた fly.toml を確かめる
B="$H/u04_bad"; rm -rf "$B"; mkdir -p "$B"
cp "$REAL/Dockerfile" "$B/"
python - "$REAL/fly.toml" "$B/fly.toml" <<'PY'
import sys
s = open(sys.argv[1], encoding="utf-8").read()
for a, b in [('  PORT = "8080"', '  PORT = "3000"'),
             ("  force_https = true", "  force_https = false"),
             ('  HOST = "0.0.0.0"', '  HOST = "0.0.0.0"\n  DATABASE_AUTH_TOKEN = "u04-fake-token"')]:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(sys.argv[2], "w", encoding="utf-8", newline="").write(s)
PY
# Fly.io と同じように、[env] の PORT=3000 を渡して動かす（中は 3000 で待ち受け、外の 18080 は 8080 へつなぐ）
docker run -d --name rn-u04 -p 18080:8080 -e PORT=3000 "$IMG" > /dev/null
sleep 3
python "$CHECK" "$B" "$P" 2>&1 | tr -d '\r' > "$O/u04_check_bad.txt"; echo "（終了コード: ${PIPESTATUS[0]}）" >> "$O/u04_check_bad.txt"
docker logs rn-u04 2>&1 | tr -d '\r' | tail -1 > "$O/u04_bad_log.txt"
docker rm -f rn-u04 > /dev/null
(cd "$B" && diff -u "$REAL/fly.toml" fly.toml | sed 's#^--- .*#--- a/fly.toml#; s#^+++ .*#+++ b/fly.toml#') | tr -d '\r' > "$O/u04_bad_diff.txt"
rm -rf "$B"
echo done

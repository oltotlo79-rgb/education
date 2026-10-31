#!/usr/bin/env bash
# 第12巻 UNIT 02 の補い: 実行用の段の Debian の版だけを変えたとき（bookworm）と、glibc の無い alpine にしたとき。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
D="$H/u02"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
ctx() {
  local c="$D/$1"; rm -rf "$c"; mkdir -p "$c"
  cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$c/"
  cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$REAL/Dockerfile" "$REAL/.dockerignore" "$c/"
  echo "$c"
}

# ---- bookworm（glibc 2.36）の実行用の段: 動くか（-d で起動して /health を見る）
docker build -q -t rustnote:glibc "$D/glibc" > /dev/null 2>&1
docker rm -f rn-glibc > /dev/null 2>&1
docker run -d --name rn-glibc -p 18080:8080 rustnote:glibc > /dev/null
for i in $(seq 1 40); do curl.exe -s -o /dev/null http://127.0.0.1:18080/health && break; sleep 0.3; done
{
  echo "実行用の段: $(docker run --rm --entrypoint sh rustnote:glibc -c 'ldd --version | head -1')"
  echo "/health: $(curl.exe -s http://127.0.0.1:18080/health)"
  echo "--- ログ"
  docker logs rn-glibc 2>&1 | tr -d '\r'
} > "$O/u02_err_glibc.txt"
docker rm -f rn-glibc > /dev/null

# ---- alpine（musl。glibc が無い）の実行用の段
A=$(ctx alpine)
python - "$A/Dockerfile" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = s[s.index("FROM debian:trixie-slim"):s.index("WORKDIR /app\n# AssetBundle")]
new = """FROM alpine:3.22

# Turso へ HTTPS でつなぐための証明書
RUN apk add --no-cache ca-certificates

# root ではない利用者で動かす。/data はローカルの DB ファイル置き場
RUN adduser -D -u 10001 app \\
    && mkdir -p /data \\
    && chown app:app /data

"""
s = s.replace(old, new)
open(p, "w", encoding="utf-8", newline="").write(s)
PY
(cd "$A" && diff -u "$REAL/Dockerfile" Dockerfile | sed 's#^--- .*#--- a/Dockerfile#; s#^+++ .*#+++ b/Dockerfile#') | tr -d '\r' > "$O/u02_err_alpine_diff.txt"
docker build --progress=plain -t rustnote:alpine "$A" > "$D/alpine.log" 2>&1
echo "exit=$?" > "$O/u02_err_alpine_build_result.txt"
docker run --rm rustnote:alpine > "$D/alpine.out" 2>&1; echo "（終了コード: $?）" >> "$D/alpine.out"
tr -d '\r' < "$D/alpine.out" > "$O/u02_err_alpine.txt"
docker run --rm --entrypoint sh rustnote:alpine -c 'ls -l /app/rustnote; ls /lib/ld-linux-aarch64.so.1 2>&1; ls /lib/ld-musl-aarch64.so.1' > "$O/u02_err_alpine_ls.txt" 2>&1
docker image rm -f rustnote:alpine rustnote:glibc > /dev/null 2>&1
echo done

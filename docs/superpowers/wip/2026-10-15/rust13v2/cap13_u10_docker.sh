#!/usr/bin/env bash
# 第13巻 UNIT 10（と第12巻）の Docker の出力を取る。
# 教材の Dockerfile（bookworm）と、Debian 13（trixie）に替えた版の両方を、実際にビルドして起動する。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
D="$H/u10docker"
rm -rf "$D"; mkdir -p "$D"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask13.py" | tr -d '\r'; }

# ---- ビルドに送る中身（.dockerignore の効き目を見るため、教材のフォルダをそのまま使う）
docker version --format 'Docker {{.Server.Version}}（{{.Server.Os}}/{{.Server.Arch}}）' > "$O/u10_docker_version.txt" 2>&1

# ---- 1. 使う4つのイメージの版を確かめる
{
  for image in rust:1.99.1-bookworm debian:bookworm-slim rust:1.99.1-trixie debian:trixie-slim; do
    docker pull -q "$image" > /dev/null 2>&1 && echo "$image: $(docker image inspect "$image" --format '{{.Architecture}}・{{.Size}}')" || echo "$image: 取得できない"
  done
} > "$O/u10_docker_images.txt"

# ---- 2. 教材の Dockerfile（bookworm）でビルド
start=$(date +%s)
docker build --progress=plain -t rustnote:bookworm "$REAL" > "$D/build_bookworm.log" 2>&1
echo "exit=$?・$(( $(date +%s) - start )) 秒" > "$O/u10_build_bookworm_result.txt"

# ---- 3. trixie に替えた版（教材のコピーの Dockerfile の2行を替える）
CTX="$D/ctx_trixie"
mkdir -p "$CTX"
cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$CTX/"
cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$REAL/Dockerfile" "$REAL/.dockerignore" "$CTX/"
python - "$CTX/Dockerfile" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
for a, b in [("FROM rust:1.99.1-bookworm AS builder", "FROM rust:1.99.1-trixie AS builder"), ("FROM debian:bookworm-slim", "FROM debian:trixie-slim")]:
    assert s.count(a) == 1, a
    s = s.replace(a, b)
open(p, "w", encoding="utf-8", newline="").write(s)
PY
start=$(date +%s)
docker build --progress=plain -t rustnote:trixie "$CTX" > "$D/build_trixie.log" 2>&1
echo "exit=$?・$(( $(date +%s) - start )) 秒" > "$O/u10_build_trixie_result.txt"

# ---- 4. 起動して確かめる
for tag in bookworm trixie; do
  {
    echo "--- rustnote:$tag"
    docker image inspect "rustnote:$tag" --format '大きさ {{.Size}} バイト・利用者 {{.Config.User}}・待ち受け {{range $p, $_ := .Config.ExposedPorts}}{{$p}}{{end}}'
    docker run --rm "rustnote:$tag" sh -c 'ldd --version | head -1; id; ls -l /app'
    docker run -d --rm --name "rn-$tag" -p 18080:8080 "rustnote:$tag" > /dev/null
    for i in $(seq 1 40); do curl.exe -s -o /dev/null "http://127.0.0.1:18080/health" && break; sleep 0.5; done
    curl.exe -s -i "http://127.0.0.1:18080/health" | tr -d '\r' | grep -E "^(HTTP|content-type)|status"
    echo "--- コンテナのログ"
    docker logs "rn-$tag" 2>&1 | tr -d '\r'
    docker stop "rn-$tag" > /dev/null
  } > "$O/u10_run_$tag.txt" 2>&1
done
docker image ls rustnote --format '{{.Repository}}:{{.Tag}} {{.Size}}' > "$O/u10_image_sizes.txt"

# ---- ビルドのログから、段ごとの時間と、送った中身の大きさを取り出す
for tag in bookworm trixie; do
  grep -E "transferring context|^#[0-9]+ \[|DONE [0-9.]+s|naming to" "$D/build_$tag.log" | mask > "$O/u10_build_${tag}_steps.txt"
done
echo "done"

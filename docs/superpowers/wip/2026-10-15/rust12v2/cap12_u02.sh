#!/usr/bin/env bash
# 第12巻 UNIT 02 の出力を取る（multi-stage の Dockerfile）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
D="$H/u02"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
rm -rf "$D"; mkdir -p "$D"
# 教材のフォルダの中身を、ビルド用のコピーに入れる（Dockerfile を差し替えて試すため）
ctx() {  # 名前
  local c="$D/$1"; mkdir -p "$c"
  cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$c/"
  cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$REAL/Dockerfile" "$REAL/.dockerignore" "$c/"
  echo "$c"
}
steps() { python "$H/steps12.py" "$1" | mask; }
build() {  # タグ・フォルダ・出力の名前（-f などは環境変数 EXTRA で）
  local tag=$1 dir=$2 name=$3 start
  start=$(date +%s)
  docker build --progress=plain ${EXTRA:-} -t "$tag" "$dir" > "$D/$name.log" 2>&1
  local code=$?
  echo "exit=$code・$(( $(date +%s) - start )) 秒" > "$O/${name}_result.txt"
  steps "$D/$name.log" > "$O/${name}_steps.txt"
  return $code
}
size() { docker image ls "$1" --format '{{.Size}}'; }

docker version --format 'Docker {{.Server.Version}}（{{.Server.Os}}/{{.Server.Arch}}）' > "$O/u02_docker_version.txt"

# ---- 例2-1: 教材の Dockerfile でビルド（キャッシュがあれば使う）
C=$(ctx real)
build rustnote:u02 "$C" u02_build || exit 1
{
  echo "rustnote:u02 の大きさ: $(size rustnote:u02)"
  echo "rust:1.99.1-trixie の大きさ: $(size rust:1.99.1-trixie)"
  echo "debian:trixie-slim の大きさ: $(size debian:trixie-slim)"
} > "$O/u02_sizes.txt"
docker history rustnote:u02 --format '{{.Size}}	{{.CreatedBy}}' | cut -c1-140 | mask > "$O/u02_history.txt"
docker run --rm rustnote:u02 sh -c 'id; ls -la /app /data; touch /app/x 2>&1; touch /data/x && echo "/data に書けた"' > "$O/u02_inside.txt" 2>&1

# ---- 例2-2: 1段だけの Dockerfile と比べる
S1=$(ctx single)
cat > "$S1/Dockerfile" <<'EOF'
# 比べるための、1段だけの Dockerfile（作る道具ごと、そのまま動かす）
FROM rust:1.99.1-trixie
WORKDIR /app
RUN cargo install topcoat-cli --version 0.11.0 --locked
COPY . .
RUN cargo build --release --locked && topcoat asset bundle --release
ENV HOST=0.0.0.0 PORT=8080 DATABASE_URL=/app/rustnote.db
CMD ["/app/target/release/rustnote"]
EOF
build rustnote:single "$S1" u02_single
{
  echo "rustnote:single の大きさ: $(size rustnote:single)"
  docker run --rm rustnote:single sh -c 'id; which cargo rustc; ls /app' 2>&1
} > "$O/u02_single_inside.txt"

# ---- 例2-3: ソースを1行変えたときの、ビルドのやり直し（教材の Dockerfile）
printf '\n// 第12巻 UNIT 02 の実験: 1行足した\n' >> "$C/src/routes/health.rs"
build rustnote:u02b "$C" u02_rebuild
# キャッシュのマウントを使う Dockerfile（問2）
M=$(ctx cachemount)
python - "$M/Dockerfile" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = """COPY . .
RUN cargo build --release --locked

# 実行ファイルの隣（target/release/assets/）に CSS の束を作る
RUN topcoat asset bundle --release
"""
assert s.count(old) == 1
new = """COPY . .
# ダウンロードしたクレートと target/ を、ビルドをまたいで取っておく（キャッシュのマウント）。
# マウントした所はイメージに残らないので、できた物は /out にコピーしておく
RUN --mount=type=cache,target=/usr/local/cargo/registry \\
    --mount=type=cache,target=/app/target \\
    cargo build --release --locked \\
    && topcoat asset bundle --release \\
    && mkdir -p /out \\
    && cp target/release/rustnote /out/rustnote \\
    && cp -r target/release/assets /out/assets
"""
s = s.replace(old, new)
s = s.replace("COPY --from=builder /app/target/release/rustnote /app/rustnote", "COPY --from=builder /out/rustnote /app/rustnote")
s = s.replace("COPY --from=builder /app/target/release/assets /app/assets", "COPY --from=builder /out/assets /app/assets")
open(p, "w", encoding="utf-8", newline="").write(s)
PY
build rustnote:cache1 "$M" u02_cache_first
printf '\n// 第12巻 UNIT 02 の実験: 1行足した\n' >> "$M/src/routes/health.rs"
build rustnote:cache2 "$M" u02_cache_second
{
  echo "rustnote:cache2 の大きさ: $(size rustnote:cache2)"
  docker run -d --rm --name rn-u02 -p 18080:8080 rustnote:cache2 > /dev/null
  for i in $(seq 1 40); do curl.exe -s -o /dev/null http://127.0.0.1:18080/health && break; sleep 0.5; done
  echo "/health: $(curl.exe -s http://127.0.0.1:18080/health)"
  docker stop rn-u02 > /dev/null
} > "$O/u02_cache_check.txt" 2>&1

# ---- エラー1: builder を trixie、実行用を bookworm にする（glibc の版が合わない）
E1=$(ctx glibc)
sed -i 's#^FROM debian:trixie-slim#FROM debian:bookworm-slim#' "$E1/Dockerfile"
build rustnote:glibc "$E1" u02_err_glibc_build
docker run --rm rustnote:glibc > "$D/glibc.out" 2>&1; echo "（終了コード: $?）" >> "$D/glibc.out"
mask < "$D/glibc.out" > "$O/u02_err_glibc.txt"
# ---- エラー2: COPY --from の場所を間違える
E2=$(ctx copypath)
sed -i 's#COPY --from=builder /app/target/release/rustnote /app/rustnote#COPY --from=builder /app/target/rustnote /app/rustnote#' "$E2/Dockerfile"
build rustnote:copypath "$E2" u02_err_copy
grep -E "ERROR|not found|failed to" "$D/u02_err_copy.log" | mask | awk '!seen[$0]++' > "$O/u02_err_copy.txt"
# ---- エラー3: /data を作らずに、root でない利用者で動かす
E3=$(ctx nodata)
python - "$E3/Dockerfile" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
old = """RUN useradd --create-home --uid 10001 app \\
    && mkdir -p /data \\
    && chown app:app /data"""
assert s.count(old) == 1
s = s.replace(old, "RUN useradd --create-home --uid 10001 app")
open(p, "w", encoding="utf-8", newline="").write(s)
PY
build rustnote:nodata "$E3" u02_err_nodata_build
docker run --rm rustnote:nodata > "$D/nodata.out" 2>&1; echo "（終了コード: $?）" >> "$D/nodata.out"
mask < "$D/nodata.out" > "$O/u02_err_nodata.txt"

# ---- 片付け（rustnote:u02 は次の単元でも使う）
docker image rm -f rustnote:single rustnote:u02b rustnote:cache1 rustnote:glibc rustnote:copypath rustnote:nodata > /dev/null 2>&1
echo done

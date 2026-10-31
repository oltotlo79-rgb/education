#!/usr/bin/env bash
# 第12巻 UNIT 05 の出力を取る（secrets・設定・入れ替え）。本物の Turso・Fly.io にはつながない。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
D="$H/u05"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask12.py" | tr -d '\r'; }
IMG=rustnote:u02
FAKE="u05-fake-token-not-a-real-secret"
rm -rf "$D"; mkdir -p "$D"

# ---- 例5-1: Turso の URL なのに、トークンを渡さない（設定の段で止まる。ネットにはつながない）
docker run --rm -e DATABASE_URL=libsql://rustnote-example.turso.io "$IMG" > "$D/a.out" 2>&1; echo "（終了コード: $?）" >> "$D/a.out"
tr -d '\r' < "$D/a.out" > "$O/u05_missing_token.txt"

# ---- 例5-2: トークンをイメージに焼き込んでしまう（ENV に書く）と、だれでも読める
mkdir -p "$D/baked"
cat > "$D/baked/Dockerfile" <<EOF
# まちがった例: 秘密の値を ENV でイメージに書き込む
FROM $IMG
ENV DATABASE_AUTH_TOKEN=$FAKE
EOF
docker build -q -t rustnote:baked "$D/baked" > /dev/null 2>&1
{
  echo "--- docker image inspect（イメージの設定の環境変数）"
  docker image inspect rustnote:baked --format '{{range .Config.Env}}{{println .}}{{end}}' | grep -v '^$'
  echo "--- docker history（作った手順の記録）"
  docker history rustnote:baked --format '{{.CreatedBy}}' --no-trunc | head -1
} | tr -d '\r' > "$O/u05_baked.txt"
docker image rm -f rustnote:baked > /dev/null 2>&1

# ---- 例5-3: 動かすときに渡す（-e）。イメージには残らない
docker run -d --name rn-u05 -e DATABASE_AUTH_TOKEN=$FAKE "$IMG" > /dev/null
sleep 2
{
  echo "--- イメージの環境変数（rustnote:u02）"
  docker image inspect "$IMG" --format '{{range .Config.Env}}{{println .}}{{end}}' | grep -v '^$'
  echo "--- 動いているコンテナの環境変数（docker inspect rn-u05）"
  docker inspect rn-u05 --format '{{range .Config.Env}}{{println .}}{{end}}' | grep -v '^$'
  echo "--- コンテナのログ"
  docker logs rn-u05 2>&1
} | tr -d '\r' > "$O/u05_runtime.txt"
docker rm -f rn-u05 > /dev/null

# ---- 例5-4: .env があるフォルダからビルドしたら、.env はどこまで届くか（.dockerignore あり・なし）
for mode in with without; do
  C="$D/ctx_$mode"; mkdir -p "$C"
  cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$C/"
  cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$REAL/Dockerfile" "$C/"
  [ "$mode" = with ] && cp "$REAL/.dockerignore" "$C/"
  printf 'DATABASE_URL=libsql://rustnote-example.turso.io\nDATABASE_AUTH_TOKEN=%s\n' "$FAKE" > "$C/.env"
  docker build -q --target builder -t "rustnote:builder-$mode" "$C" > /dev/null 2>&1
  docker build -q -t "rustnote:final-$mode" "$C" > /dev/null 2>&1
  {
    echo "--- .dockerignore $( [ "$mode" = with ] && echo あり || echo なし )"
    echo "builder の段の /app/.env: $(docker run --rm "rustnote:builder-$mode" sh -c 'test -f /app/.env && echo ある || echo 無い')"
    docker run --rm "rustnote:builder-$mode" sh -c 'test -f /app/.env && grep -c TOKEN /app/.env | sed "s/^/  （トークンの行: /; s/$/ 行）/"'
    echo "できたイメージの /app/.env: $(docker run --rm "rustnote:final-$mode" sh -c 'test -f /app/.env && echo ある || echo 無い')"
  } | tr -d '\r' >> "$O/u05_dotenv.txt"
done
docker image rm -f rustnote:builder-with rustnote:builder-without rustnote:final-with rustnote:final-without > /dev/null 2>&1

rm -rf "$D"
echo done

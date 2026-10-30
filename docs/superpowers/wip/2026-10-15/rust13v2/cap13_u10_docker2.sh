#!/usr/bin/env bash
# 第13巻 UNIT 10 のエラーと練習問題の出力を取る（Docker のイメージを作ったあとに動かす）。
# 使い方: bash cap13_u10_docker2.sh <使うタグ: bookworm | trixie>
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
TAG=$1
IMAGE="rustnote:$TAG"
O="$H/out"
D="$H/u10docker"
CAP="bash $H/cap.sh"
export PYTHONIOENCODING=utf-8
P=http://127.0.0.1:18080
wait_up() { for i in $(seq 1 40); do curl.exe -s -o /dev/null "$P/health" && return 0; sleep 0.5; done; return 1; }
SIGNUP="email=alice%40example.com&password=correct+horse+battery&password_confirm=correct+horse+battery"
LOGIN="email=alice%40example.com&password=correct+horse+battery"
status() { curl.exe -s -o /dev/null -w "%{http_code}" "$@"; }

# ---- エラー1: Cargo.toml だけ変えて、Cargo.lock を直さずに --locked でビルドする
L="$H/u10locked"
rm -rf "$L"; mkdir -p "$L"
cp -r "$REAL/src" "$REAL/tests" "$REAL/migrations" "$REAL/assets" "$L/"
cp "$REAL/Cargo.toml" "$REAL/Cargo.lock" "$REAL/rust-toolchain.toml" "$REAL/rustfmt.toml" "$L/"
python - "$L/Cargo.toml" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = 'tempfile = "=3.27.0"'
assert s.count(a) == 1
open(p, "w", encoding="utf-8", newline="").write(s.replace(a, 'tempfile = "=3.26.0"'))
PY
WORKDIR=u10locked $CAP u10_err_locked cargo build --release --locked > /dev/null
rm -rf "$L"

# ---- エラー2: 実行用のイメージに assets/ をコピーし忘れる
C2="$D/ctx_noassets"
rm -rf "$C2"; cp -r "$D/ctx_$TAG" "$C2" 2>/dev/null || { mkdir -p "$C2"; cp -r "$REAL/." "$C2/"; }
python - "$C2/Dockerfile" <<'PY'
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
a = "COPY --from=builder /app/target/release/assets /app/assets\n"
assert s.count(a) == 1
open(p, "w", encoding="utf-8", newline="").write(s.replace(a, ""))
PY
docker build -q -t rustnote:noassets "$C2" > /dev/null 2>&1
{
  docker run --rm rustnote:noassets 2>&1 | tr -d '\r'
  echo "（終了コード: ${PIPESTATUS[0]}）"
} > "$O/u10_err_noassets.txt"
docker image rm -f rustnote:noassets > /dev/null 2>&1

# ---- エラー3: コンテナの中のポート（8080）とちがう番号（3000）につなぐ
docker run -d --rm --name rn-port -p 18080:3000 "$IMAGE" > /dev/null
sleep 3
{
  code=$(curl.exe -s -o /dev/null -w "%{http_code}" "$P/health"); echo "状態 $code・curl の終了コード $?"
  echo "--- コンテナのログ"
  docker logs rn-port 2>&1 | tr -d '\r'
} > "$O/u10_err_port.txt"
docker stop rn-port > /dev/null

# ---- 問2: コンテナを作り直すと、登録したユーザーはどうなるか（volume なし・あり）
run_q2() {  # 名前の頭・docker run の追加の引数…
  local name=$1; shift
  docker run -d --rm --name rn-q2 -p 18080:8080 "$@" "$IMAGE" > /dev/null; wait_up
  echo "1回目のコンテナ: 登録 $(status -d "$SIGNUP" "$P/signup")・ログイン $(status -d "$LOGIN" "$P/login")"
  docker stop rn-q2 > /dev/null
  docker run -d --rm --name rn-q2 -p 18080:8080 "$@" "$IMAGE" > /dev/null; wait_up
  echo "作り直したコンテナ: ログイン $(status -d "$LOGIN" "$P/login")"
  docker logs rn-q2 2>&1 | tr -d '\r' | grep -E "マイグレーション|起動しました" | sed 's/^/  ログ: /'
  docker stop rn-q2 > /dev/null
}
{
  echo "--- volume なし"
  run_q2 none
  docker volume rm -f rustnote-u10 > /dev/null 2>&1
  echo "--- volume あり（-v rustnote-u10:/data）"
  run_q2 vol -v rustnote-u10:/data
  docker volume rm -f rustnote-u10 > /dev/null 2>&1
} > "$O/u10_q2.txt" 2>&1

# ---- 問3: HOST=127.0.0.1 で動かす
docker run -d --rm --name rn-host -p 18080:8080 -e HOST=127.0.0.1 "$IMAGE" > /dev/null
sleep 3
{
  code=$(curl.exe -s -o /dev/null -w "%{http_code}" "$P/health"); echo "状態 $code・curl の終了コード $?"
  echo "--- コンテナのログ"
  docker logs rn-host 2>&1 | tr -d '\r'
} > "$O/u10_q3.txt"
docker stop rn-host > /dev/null
echo done

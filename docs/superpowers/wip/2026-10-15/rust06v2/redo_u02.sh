#!/usr/bin/env bash
# 第6巻 UNIT 02 の出力を Topcoat 0.11.0 / Rust 1.99.1 / tokio 1.54.1 で取り直す
set -u
S="$(cd "$(dirname "$0")/.." && pwd)"
V="$S/rust06v11"
OLD="$S/rust06/web_lab"
OUT="$V/out"
mkdir -p "$OUT"
mask() { PYTHONIOENCODING=utf-8 python "$S/maskx.py" | tr -d '\r'; }
save() { mask > "$OUT/$1.txt"; }

stop_all() {
  for exe in web_lab.exe u02_serve.exe u02_q1.exe; do
    taskkill //F //IM "$exe" > /dev/null 2>&1
  done
  sleep 1
}

stop_all
cd "$V" || exit 1
rm -rf web_lab
cargo new -q web_lab
cd web_lab
printf '[toolchain]\nchannel = "1.99.1"\n' > rust-toolchain.toml
cp "$OLD/rustfmt.toml" .
export CARGO_TARGET_DIR="$V/target-fresh"
rm -rf "$CARGO_TARGET_DIR"

# 例2-1: cargo add
{ cargo add topcoat@=0.11.0 2>&1; echo "exit=$?"; } | save u02_add_topcoat
{ cargo add tokio@=1.54.1 --features rt-multi-thread,macros 2>&1; echo "exit=$?"; } | save u02_add_tokio

# 例2-2: main.rs（コメントの版だけ 0.11 に）
sed 's/Topcoat 0\.10 の/Topcoat 0.11 の/' "$OLD/src/main.rs" > src/main.rs

# 新しい target で、最初のビルド（Compiling の行数と時間）
{ cargo build --bin web_lab 2>&1; echo "exit=$?"; } | save u02_build_fresh
grep -c "^   Compiling" "$OUT/u02_build_fresh.txt" > "$OUT/u02_build_fresh_count.txt"
grep -E "^   Compiling web_lab|^    Finished" "$OUT/u02_build_fresh.txt" > "$OUT/u02_build_fresh_tail.txt"

BIN="$CARGO_TARGET_DIR/debug"
start_server() { # $1 = exe 名, 残りは環境変数
  local exe="$1"; shift
  (cd "$V/web_lab" && env "$@" "$BIN/$exe" > "$OUT/server_$exe.log" 2>&1 &)
  sleep 2
}

start_server web_lab.exe
/c/Windows/System32/curl.exe -s -i http://127.0.0.1:3000/ | save u02_home
{
  echo '### curl.exe -i http://127.0.0.1:3000/nothing'
  /c/Windows/System32/curl.exe -s -i http://127.0.0.1:3000/nothing; echo; echo
  echo '### curl.exe -i -X POST http://127.0.0.1:3000/'
  /c/Windows/System32/curl.exe -s -i -X POST http://127.0.0.1:3000/; echo; echo
  echo '### curl.exe -I http://127.0.0.1:3000/'
  /c/Windows/System32/curl.exe -s -I http://127.0.0.1:3000/
} | save u02_status
stop_all

# 例2-3: net を足して u02_serve
{ cargo add tokio@=1.54.1 --features rt-multi-thread,macros,net 2>&1; echo "exit=$?"; } | save u02_add_net
mkdir -p src/bin
cp "$OLD/src/bin/u02_serve.rs" src/bin/
cargo build -q --bin u02_serve 2>&1 | mask
start_server u02_serve.exe
"$BIN/u02_serve.exe" 2>&1 | save u02_serve2_err
cat "$OUT/server_u02_serve.exe.log" | save u02_serve1
stop_all

# エラー1: .await を消す
MAIN_OK="$V/main_ok.rs"; cp src/main.rs "$MAIN_OK"
python - "$MAIN_OK" src/main.rs <<'PY'
import sys
s = open(sys.argv[1], encoding="utf-8").read()
a = "    topcoat::start(router)\n        .await\n        .expect("
assert s.count(a) == 1
open(sys.argv[2], "w", encoding="utf-8", newline="\n").write(s.replace(a, "    topcoat::start(router)\n        .expect("))
PY
{ cargo build 2>&1; echo "exit=$?"; } | save u02_err_noawait
cp "$MAIN_OK" src/main.rs

# エラー2: RouterBuilderDiscoverExt を消す
python - "$MAIN_OK" src/main.rs <<'PY'
import sys
s = open(sys.argv[1], encoding="utf-8").read()
a = "router::{Router, RouterBuilderDiscoverExt, page},"
assert s.count(a) == 1
open(sys.argv[2], "w", encoding="utf-8", newline="\n").write(s.replace(a, "router::{Router, page},"))
PY
{ cargo build --bin web_lab 2>&1; echo "exit=$?"; } | save u02_err_discover
cp "$MAIN_OK" src/main.rs

# エラー3: PORT=abc
{ PORT=abc cargo run -q --bin web_lab 2>&1; echo "exit=$?"; } | save u02_err_port

# エラー4: bin が2つ
{ cargo run 2>&1; echo "exit=$?"; } | save u02_err_whichbin

# 問1: /about
cp "$OLD/src/bin/u02_q1.rs" src/bin/
cargo build -q --bin u02_q1 --bin web_lab 2>&1 | mask
start_server u02_q1.exe
{
  echo '### curl.exe -i http://127.0.0.1:3000/about'
  /c/Windows/System32/curl.exe -s -i http://127.0.0.1:3000/about
} | save u02_q1_curl
# 問3: --compressed（ヘッダーだけ）
/c/Windows/System32/curl.exe -s --compressed -D - -o NUL http://127.0.0.1:3000/about | save u02_q3_compressed
stop_all

# 問2: PORT=3001
start_server web_lab.exe PORT=3001
{
  echo '### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3001/'
  /c/Windows/System32/curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3001/; echo
  echo '### curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/'
  /c/Windows/System32/curl.exe -s -o NUL -w "%{http_code}" http://127.0.0.1:3000/; code=$?; echo
  echo "(curl の終了コード: $code)"
} | save u02_q2
stop_all

cargo clippy -q --all-targets -- -D warnings 2>&1 | mask
cargo fmt --check && echo fmt-ok
tasklist | grep -iE "web_lab|u02_serve|u02_q1" || echo "サーバーは残っていない"

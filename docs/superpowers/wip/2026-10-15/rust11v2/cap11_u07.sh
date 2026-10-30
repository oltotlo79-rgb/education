#!/usr/bin/env bash
# 第11巻 UNIT 07 の出力を取る（RustNote のコピーに tracing と /metrics を足した版）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11b"
# 練習用のコピーをもう1つ作って使う（rustnote は別の実験で使っているため）
export WORKDIR=rustnote_u07
CAP="bash $S/capx.sh"
E="python $S/rust10/edit_src.py"
X="$S/rust11/errs"
O="$S/rust11/out"
ST="$S/rust11/staging_u07"
cd "$S/rust11/rustnote_u07" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
cp Cargo.toml "$S/rust11/Cargo.toml.u07keep"; cp Cargo.lock "$S/rust11/Cargo.lock.u07keep"

# 1. tracing を足す
$CAP u07_add cargo add tracing@=0.1.44 > /dev/null
$CAP u07_add2 cargo add tracing-subscriber@=0.3.23 --features json,env-filter > /dev/null
sed -n '/^\[dependencies\]/,/^\[dev-dependencies\]/p' Cargo.toml > "$O/u07_cargo_deps.txt"

# 2. /metrics と、アクセスログ・500 のログ・起動のときの subscriber
cp "$S/rust11/variants/metrics.rs" src/routes/metrics.rs
for pair in "src/routes.rs:e07_routes" "src/views.rs:e07_views" "src/main.rs:e07_main"; do
  $E "${pair%%:*}" "$X/${pair##*:}.py" > /dev/null || exit 1
done
for f in routes views main; do
  src="src/$f.rs"
  diff -u --label "a/$src" --label "b/$src" "$REAL/$src" "$src" > "$O/u07_diff_$f.txt"
done
cargo build -q --tests || { echo "組み立てに失敗"; exit 1; }

for t in u07_logs u07_slo e07_no_subscriber e07_init_twice e07_static; do cp "$ST/$t.rs" tests/; done
$CAP u07_logs cargo test --test u07_logs -- --nocapture > /dev/null
$CAP u07_slo cargo test --test u07_slo -- --nocapture > /dev/null
$CAP u07_err_none cargo test --test e07_no_subscriber -- --nocapture > /dev/null
$CAP u07_err_twice cargo test --test e07_init_twice > /dev/null
$CAP u07_err_static cargo test --test e07_static > /dev/null

# 3. 本物のサーバーで、起動のときの subscriber と /metrics を見る
cargo build -q && topcoat asset bundle > /dev/null
DB="$(cygpath -w "$S/rust11/u07_server.db")"; rm -f "$S/rust11/u07_server.db"*
( HOST=127.0.0.1 PORT=3012 DATABASE_URL="$DB" "$S/target11b/debug/rustnote.exe" > "$O/u07_server_stdout.raw" 2> "$O/u07_server_stderr.raw" & echo $! > "$S/rust11/u07_server.pid" )
for i in $(seq 1 30); do curl.exe -s -o /dev/null http://127.0.0.1:3012/health && break; sleep 0.5; done
curl.exe -s -o /dev/null http://127.0.0.1:3012/login
curl.exe -s -o /dev/null http://127.0.0.1:3012/notes
curl.exe -s -o /dev/null http://127.0.0.1:3012/nothing-here
curl.exe -s http://127.0.0.1:3012/metrics > "$O/u07_server_metrics.txt"
curl.exe -s -i http://127.0.0.1:3012/metrics | head -4 | tr -d '\r' > "$O/u07_server_metrics_head.txt"
kill "$(cat "$S/rust11/u07_server.pid")" 2> /dev/null; taskkill //F //IM rustnote.exe > /dev/null 2>&1
sleep 1
PYTHONIOENCODING=utf-8 python "$S/maskx.py" < "$O/u07_server_stdout.raw" | tr -d '\r' > "$O/u07_server_stdout.txt"
PYTHONIOENCODING=utf-8 python "$S/maskx.py" < "$O/u07_server_stderr.raw" | tr -d '\r' > "$O/u07_server_stderr.txt"
rm -f "$S/rust11/u07_server.db"* "$S/rust11/u07_server.pid"

# 問1: RUST_LOG=warn で起動すると、info のアクセスログは出ない
( RUST_LOG=warn HOST=127.0.0.1 PORT=3012 DATABASE_URL="$DB" "$S/target11b/debug/rustnote.exe" > "$O/u07_warn_stdout.raw" 2> "$O/u07_warn_stderr.raw" & echo $! > "$S/rust11/u07_server.pid" )
for i in $(seq 1 30); do curl.exe -s -o /dev/null http://127.0.0.1:3012/health && break; sleep 0.5; done
curl.exe -s -o /dev/null http://127.0.0.1:3012/login
curl.exe -s -o /dev/null http://127.0.0.1:3012/nothing-here
kill "$(cat "$S/rust11/u07_server.pid")" 2> /dev/null; taskkill //F //IM rustnote.exe > /dev/null 2>&1
sleep 1
PYTHONIOENCODING=utf-8 python "$S/maskx.py" < "$O/u07_warn_stdout.raw" | tr -d '\r' > "$O/u07_warn_stdout.txt"
PYTHONIOENCODING=utf-8 python "$S/maskx.py" < "$O/u07_warn_stderr.raw" | tr -d '\r' > "$O/u07_warn_stderr.txt"
rm -f "$S/rust11/u07_server.db"* "$S/rust11/u07_server.pid"

# 4. UNIT 04 の直し方（題の制御文字を断る）も当てた版で、SLO をもう一度
$E src/domain.rs "$X/e04_no_control.py" > /dev/null
$CAP u07_slo_fixed cargo test --test u07_slo -- --nocapture > /dev/null

# 元に戻す
for t in u07_logs u07_slo e07_no_subscriber e07_init_twice e07_static; do rm "tests/$t.rs"; done
rm src/routes/metrics.rs
for f in src/routes.rs src/views.rs src/main.rs src/domain.rs; do cp "$REAL/$f" "$f"; done
cp "$S/rust11/Cargo.toml.u07keep" Cargo.toml; cp "$S/rust11/Cargo.lock.u07keep" Cargo.lock
rm "$S/rust11/Cargo.toml.u07keep" "$S/rust11/Cargo.lock.u07keep"
check
cargo build -q --tests

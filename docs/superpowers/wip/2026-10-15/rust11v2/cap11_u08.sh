#!/usr/bin/env bash
# 第11巻 UNIT 08 の出力を取る（criterion のベンチマーク・部分ごとの時間・メモリ）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
export VOL=rust11
export CARGO_TARGET_DIR="$S/target11"
CAP="bash $S/capx.sh"
O="$S/rust11/out"
ST="$S/rust11/staging_u08"
cd "$S/rust11/rustnote" || exit 1
check() { diff -rq src "$REAL/src" > /dev/null && diff -rq migrations "$REAL/migrations" > /dev/null && echo "src と migrations は教材と同じ" || { echo "教材と違う"; exit 1; }; }
check
cp Cargo.toml "$S/rust11/Cargo.toml.u08keep"; cp Cargo.lock "$S/rust11/Cargo.lock.u08keep"

# 1. criterion を足し、benches/parse.rs を置く
$CAP u08_add cargo add --dev criterion@=0.8.2 > /dev/null
printf '\n[[bench]]\nname = "parse"\nharness = false\n' >> Cargo.toml
tail -4 Cargo.toml > "$O/u08_bench_toml.txt"
mkdir -p benches && cp "$ST/parse.rs" benches/parse.rs
$CAP u08_bench cargo bench --bench parse > /dev/null

# エラー3: harness = false を書き忘れた
cp "$ST/parse.rs" benches/parse_noharness.rs
printf '\n[[bench]]\nname = "parse_noharness"\n' >> Cargo.toml
$CAP u08_err_harness cargo bench --bench parse_noharness > /dev/null

# 2. ログインの部分ごとの時間（debug と release）
for t in u08_login_parts u08_memory u08_debug_vs_release; do cp "$ST/$t.rs" tests/; done
$CAP u08_login_debug cargo test --test u08_login_parts -- --nocapture > /dev/null
$CAP u08_login_release cargo test --release --test u08_login_parts -- --nocapture > /dev/null

# エラー1: debug で速さを測る
$CAP u08_err_debug cargo test --test u08_debug_vs_release -- --nocapture > /dev/null
$CAP u08_err_release cargo test --release --test u08_debug_vs_release -- --nocapture > /dev/null

# 3. メモリ
$CAP u08_memory cargo test --release --test u08_memory -- --nocapture --test-threads=1 > /dev/null

# 元に戻す
for t in u08_login_parts u08_memory u08_debug_vs_release; do rm "tests/$t.rs"; done
rm -rf benches
cp "$S/rust11/Cargo.toml.u08keep" Cargo.toml; cp "$S/rust11/Cargo.lock.u08keep" Cargo.lock
rm "$S/rust11/Cargo.toml.u08keep" "$S/rust11/Cargo.lock.u08keep"
check

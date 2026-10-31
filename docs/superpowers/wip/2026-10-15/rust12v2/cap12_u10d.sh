#!/usr/bin/env bash
# 第12巻 UNIT 10: 依存の更新を、別のフォルダで試してからテストする。版を書く場所がそろっているかも確かめる。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust12"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
export PYTHONIOENCODING=utf-8
W="$H/u10d"; rm -rf "$W"; mkdir -p "$W/upd"
( cd "$REAL" && cp -r src migrations assets tests Cargo.toml Cargo.lock rust-toolchain.toml Dockerfile "$W/upd/" )
cd "$W/upd" || exit 1
cargo update 2>&1 | tr -d '\r' > "$O/u10_update.txt"
diff -u "$REAL/Cargo.lock" Cargo.lock | tr -d '\r' | grep -E '^[-+][^-+]' > "$O/u10_lock_diff.txt"
find src tests -type f -exec touch {} +
start=$(date +%s)
CARGO_TARGET_DIR="$H/target" cargo test -q 2>&1 | tr -d '\r' | grep -E "^test result|FAILED|panicked" > "$O/u10_update_test.txt"
echo "（cargo test にかかった時間: $(( $(date +%s) - start )) 秒）" >> "$O/u10_update_test.txt"
cargo search libsql --limit 1 2>&1 | tr -d '\r' | head -1 > "$O/u10_search_libsql.txt"
# 版を書いている場所
{
  echo "rust-toolchain.toml: $(grep '^channel' "$REAL/rust-toolchain.toml")"
  echo "Dockerfile:          $(grep '^FROM rust' "$REAL/Dockerfile")"
  echo "Dockerfile:          $(grep 'cargo install topcoat-cli' "$REAL/Dockerfile" | sed 's/^ *//')"
  echo "Cargo.toml:          $(grep '^topcoat = ' "$REAL/Cargo.toml")"
  echo "Cargo.toml:          $(grep '^rust-version' "$REAL/Cargo.toml")"
} | tr -d '\r' > "$O/u10_versions.txt"
echo done

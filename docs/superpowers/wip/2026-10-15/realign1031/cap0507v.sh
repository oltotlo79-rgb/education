#!/usr/bin/env bash
# 第5巻・第7巻の、cargo add の出力（版の番号と Locking の行が出る）を、Rust 1.99.1・tokio 1.54.1・rust-version "1.99" で、本と同じ順番で取り直す。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust04v"
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
O="$H/out"; mkdir -p "$O"
export PYTHONIOENCODING=utf-8
mask() { python "$S/maskx.py" | tr -d '\r'; }
setrv() {  # Cargo.toml の edition の次の行に rust-version = "1.99" を足す
  python - "$1/Cargo.toml" <<'PY'
import sys
p = sys.argv[1]; s = open(p, encoding="utf-8").read()
assert s.count('edition = "2024"\n') == 1
open(p, "w", encoding="utf-8", newline="\n").write(s.replace('edition = "2024"\n', 'edition = "2024"\nrust-version = "1.99"\n'))
PY
}
cd "$H" || exit 1
rm -rf w5 w7; mkdir w5 w7

# ================= 第5巻 async_lab
cd "$H/w5"
cargo new -q async_lab
cd async_lab
cargo add tokio@=1.54.1 --features full 2>&1 | mask > "$O/v05_add_tokio.txt"
setrv .
cat Cargo.toml | tr -d '\r' > "$O/v05_cargo_toml.txt"
cargo build 2>&1 | mask > "$O/v05_first_build.txt"
cargo add tokio-util@=0.7.20 --features rt 2>&1 | mask > "$O/v05_add_tokio_util.txt"
cargo add argon2@=0.6.0 2>&1 | mask > "$O/v05_add_argon2.txt"
{ cargo add reqwest@=0.13.5 --features json; cargo add serde@=1.0.229 --features derive; } 2>&1 | mask > "$O/v05_add_reqwest_serde.txt"
{ cargo add tracing@=0.1.44; cargo add tracing-subscriber@=0.3.23 --features env-filter,json; } 2>&1 | mask > "$O/v05_add_tracing.txt"
cargo add libsql@=0.10.0 2>&1 | mask > "$O/v05_add_libsql.txt"
cat Cargo.toml | tr -d '\r' > "$O/v05_cargo_toml_final.txt"

# ================= 第7巻 sql-lab
cd "$H/w7"
cargo new sql-lab 2>&1 | mask > "$O/v07_new.txt"
cd sql-lab
{ cargo add libsql@0.10.0; cargo add tokio@1.54.1 --features rt-multi-thread,macros; cargo add thiserror@2.0.21; } 2>&1 | mask > "$O/v07_add.txt"
setrv .
cat Cargo.toml | tr -d '\r' > "$O/v07_cargo_toml.txt"
cargo add async-trait@0.1.92 2>&1 | mask > "$O/v07_add_async_trait.txt"
echo done

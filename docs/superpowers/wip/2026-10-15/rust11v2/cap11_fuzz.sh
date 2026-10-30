#!/usr/bin/env bash
# 第11巻 UNIT 04 の fuzz の出力を、WSL（Ubuntu・aarch64）で取る。
# Windows 側から: wsl.exe -d Ubuntu -- bash -l <この .sh の WSL のパス>
set -u
LAB=/mnt/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust11/fuzz_lab
OUT=$LAB/../out
# ビルド先は WSL の中（/mnt/c の上は遅いため）
export CARGO_TARGET_DIR="$HOME/rust11-fuzz-target"
cd "$LAB" || exit 1
# 出力の中の、自分のホームフォルダと作業フォルダを短くする
mask() { sed -e "s|$HOME|~|g" -e "s|$LAB|fuzz_lab|g" -e 's/\r//'; }
rm -rf fuzz/artifacts fuzz/corpus

{ echo '$ cargo +nightly fuzz --version'; cargo +nightly fuzz --version; echo '$ rustc +nightly --version'; rustc +nightly --version; uname -m; } 2>&1 | mask > "$OUT/u04_fuzz_versions.txt"

# 1. parse の fuzz を 60 秒
cargo +nightly fuzz build parsers 2>&1 | tail -3 | mask > "$OUT/u04_fuzz_build.txt"
cargo +nightly fuzz run parsers -- -max_total_time=60 2>&1 | mask > "$OUT/u04_fuzz_parsers.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u04_fuzz_parsers.txt"

# 2. NoteQuery の offset の fuzz（見つかったら、そこで止まる）
cargo +nightly fuzz run query_offset 2>&1 | mask > "$OUT/u04_fuzz_query.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u04_fuzz_query.txt"
crash=$(ls fuzz/artifacts/query_offset/ | head -1)
{ echo "\$ cargo +nightly fuzz fmt query_offset fuzz/artifacts/query_offset/$crash"; cargo +nightly fuzz fmt query_offset "fuzz/artifacts/query_offset/$crash"; echo "\$ xxd fuzz/artifacts/query_offset/$crash"; xxd "fuzz/artifacts/query_offset/$crash"; } 2>&1 | mask > "$OUT/u04_fuzz_fmt.txt"
ls fuzz/corpus/parsers | wc -l > "$OUT/u04_fuzz_corpus_count.txt"

# 3. 問3: NoteQuery::new で作る版を 30 秒
cargo +nightly fuzz run query_new -- -max_total_time=30 2>&1 | mask > "$OUT/u04_fuzz_query_new.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u04_fuzz_query_new.txt"

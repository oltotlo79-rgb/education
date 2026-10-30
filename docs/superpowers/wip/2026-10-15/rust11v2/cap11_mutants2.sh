#!/usr/bin/env bash
# 第11巻 UNIT 06: cargo-mutants で src/repository/notes.rs をわざと壊す（WSL で動かす）。
set -u
COPY=/mnt/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust11/rustnote_mut
OUT=$COPY/../out
mask() { sed -e "s|$HOME|~|g" -e "s|$COPY|rustnote|g" -e 's/\r//'; }
cd "$COPY" || exit 1
rm -rf mutants.out mutants.out.old

# 1. HTTP のテスト（tests/http.rs と u06_attacks）だけで
cargo mutants --file src/repository/notes.rs --timeout 300 -- --test http --test u06_attacks 2>&1 | mask > "$OUT/u06_mut_notes_http.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u06_mut_notes_http.txt"
for k in caught missed unviable; do cp "mutants.out/$k.txt" "$OUT/u06_mut_notes_http_$k.txt" 2>/dev/null; done
rm -rf mutants.out mutants.out.old

# 2. リポジトリの結合テスト（tests/repository.rs）と、ユニットテスト（--lib）も足して
cargo mutants --file src/repository/notes.rs --timeout 300 -- --lib --test http --test u06_attacks --test repository 2>&1 | mask > "$OUT/u06_mut_notes_all.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u06_mut_notes_all.txt"
for k in caught missed unviable; do cp "mutants.out/$k.txt" "$OUT/u06_mut_notes_all_$k.txt" 2>/dev/null; done
rm -rf mutants.out mutants.out.old

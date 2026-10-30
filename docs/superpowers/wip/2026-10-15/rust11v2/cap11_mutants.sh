#!/usr/bin/env bash
# 第11巻 UNIT 06: cargo-mutants で src/auth.rs をわざと壊し、テストが気づくかを見る（WSL で動かす）。
# Windows 側から: wsl -d Ubuntu -- bash -l <この .sh の WSL のパス>
set -u
COPY=/mnt/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust11/rustnote_mut
OUT=$COPY/../out
mask() { sed -e "s|$HOME|~|g" -e "s|$COPY|rustnote|g" -e 's/\r//'; }
cd "$COPY" || exit 1
# rust-toolchain.toml の 1.99.1 を、WSL にも入れておく
rustup toolchain install 1.99.1 --profile minimal -c rustfmt -c clippy 2>&1 | tail -1
rm -rf mutants.out mutants.out.old

{ echo '$ cargo mutants --version'; cargo mutants --version; echo '$ cargo mutants --list --file src/auth.rs'; cargo mutants --list --file src/auth.rs; } 2>&1 | mask > "$OUT/u06_mutants_list.txt"

# 1. RustNote の tests/http.rs だけで
cargo mutants --file src/auth.rs --timeout 300 -- --test http 2>&1 | mask > "$OUT/u06_mutants_http.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u06_mutants_http.txt"
cp mutants.out/missed.txt "$OUT/u06_mutants_http_missed.txt" 2>/dev/null
cp mutants.out/unviable.txt "$OUT/u06_mutants_http_unviable.txt" 2>/dev/null
rm -rf mutants.out mutants.out.old

# 2. tests/http.rs と、この単元の u06_attacks で
cargo mutants --file src/auth.rs --timeout 300 -- --test http --test u06_attacks 2>&1 | mask > "$OUT/u06_mutants_both.txt"
echo "exit=${PIPESTATUS[0]}" >> "$OUT/u06_mutants_both.txt"
cp mutants.out/missed.txt "$OUT/u06_mutants_both_missed.txt" 2>/dev/null
rm -rf mutants.out mutants.out.old

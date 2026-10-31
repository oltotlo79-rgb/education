#!/usr/bin/env bash
# 第1巻 UNIT 06 の出力を取る（Cargo の品質コマンド）。小さな練習は rsb の Rust、RustNote の時間はいつもの Rust で測る。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
R=/c/Users/oltot/rsb
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1; 'exit=' + \$LASTEXITCODE" 2>&1 | mask; }
Q='C:\Users\oltot\rsb\w\quality'

rm -rf "$R/w/quality"
psb "Set-Location C:\Users\oltot\rsb\w; cargo new --lib quality" > "$O/u06_new.txt"
cp "$H/u06src/lib_before.rs" "$R/w/quality/src/lib.rs"

# ---- 例6-1: check（型の確かめだけ）
psb "Set-Location $Q; cargo check" > "$O/u06_check.txt"
# ---- 例6-2: fmt（整形）
psb "Set-Location $Q; cargo fmt --check" > "$O/u06_fmt_check.txt"
psb "Set-Location $Q; cargo fmt; cargo fmt --check" > "$O/u06_fmt_after.txt"
tr -d '\r' < "$R/w/quality/src/lib.rs" | sed -n 1,9p > "$O/u06_lib_formatted.txt"
# ---- 例6-3: clippy（よくある書き方の注意）
psb "Set-Location $Q; cargo clippy" > "$O/u06_clippy.txt"
psb "Set-Location $Q; cargo clippy -- -D warnings" > "$O/u06_clippy_deny.txt"
# ---- 例6-4: test
psb "Set-Location $Q; cargo test" > "$O/u06_test_fail.txt"
# ---- 直したあと、4つを順に
cp "$H/u06src/lib_after.rs" "$R/w/quality/src/lib.rs"
psb "Set-Location $Q; cargo fmt --check; 'fmt: ' + \$LASTEXITCODE; cargo clippy -q -- -D warnings; 'clippy: ' + \$LASTEXITCODE; cargo test -q; 'test: ' + \$LASTEXITCODE" > "$O/u06_all_ok.txt"

# ---- エラー: 部品（rustfmt・clippy）が入っていない Rust
psb "rustup toolchain install 1.99.1 --profile minimal" > "$O/u06_minimal_install.txt"
psb "Set-Location $Q; cargo +1.99.1 fmt --check" > "$O/u06_fmt_missing.txt"
psb "Set-Location $Q; cargo +1.99.1 clippy" > "$O/u06_clippy_missing.txt"
psb "rustup component add rustfmt clippy --toolchain 1.99.1" > "$O/u06_component_add.txt"
psb "Set-Location $Q; cargo +1.99.1 fmt --check" > "$O/u06_fmt_fixed.txt"

# ---- 例6-5: RustNote で check と build の時間を比べる（いつもの Rust。target は空から）
REAL='C:\Users\oltot\Documents\git-projects\def\Rust\rustnote'
T1=$(cygpath -w "$H/t06check"); T2=$(cygpath -w "$H/t06build")
rm -rf "$H/t06check" "$H/t06build"
powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location $REAL; \$env:CARGO_TARGET_DIR = '$T1'; \$t = Measure-Command { cargo check --locked 2>&1 | Select-Object -Last 1 | Out-Host }; 'cargo check: ' + [math]::Round(\$t.TotalSeconds, 1) + ' s'; \$env:CARGO_TARGET_DIR = '$T2'; \$t = Measure-Command { cargo build --locked 2>&1 | Select-Object -Last 1 | Out-Host }; 'cargo build: ' + [math]::Round(\$t.TotalSeconds, 1) + ' s'; 'rustnote.exe in check target: ' + @(Get-ChildItem '$T1' -Recurse -Filter rustnote.exe).Count ; 'rustnote.exe in build target: ' + @(Get-ChildItem '$T2' -Recurse -Filter rustnote.exe).Count " 2>&1 | python "$S/maskx.py" | tr -d '\r' > "$O/u06_rustnote_time.txt"
echo done

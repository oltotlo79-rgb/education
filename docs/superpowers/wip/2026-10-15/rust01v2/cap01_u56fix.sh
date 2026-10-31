#!/usr/bin/env bash
# 第1巻 UNIT 04〜06 の取り直し（その2）。PowerShell は必ずホームフォルダから始める
# （RustNote のフォルダから始めると、その rust-toolchain.toml の 1.99.1 が効いてしまうため）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
cd ~ || exit 1
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location ~; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
psbx() { psb "$1; 'exit=' + \$LASTEXITCODE"; }
HELLO='C:\Users\oltot\rust-study\hello'
Q='C:\Users\oltot\rust-study\quality'

# ---- UNIT 04: ホームフォルダで
psb "rustc --version; cargo --version; rustup --version; where.exe rustc cargo rustup" > "$O/u04_versions.txt"
psb "rustup show" > "$O/u04_show.txt"
psb "rustup component list --installed" > "$O/u04_components.txt"

# ---- UNIT 05: stable から x64 の部品を外してから
echo "--- target remove"; psb "rustup target remove x86_64-pc-windows-msvc; rustup target list --installed"
rm -rf /c/Users/oltot/rust-study/hello/target/x86_64-pc-windows-msvc
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc" > "$O/u05_target_missing.txt"
psb "rustup target add x86_64-pc-windows-msvc" > "$O/u05_target_add.txt"
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc; & ..\machine.ps1 target\debug\hello.exe target\x86_64-pc-windows-msvc\debug\hello.exe; .\target\x86_64-pc-windows-msvc\debug\hello.exe" > "$O/u05_x64_build.txt"
psb "rustup target list --installed" > "$O/u05_target_list.txt"
psb "rustc -vV" > "$O/u05_rustc_vv.txt"

# ---- UNIT 06: まだ入れていない 1.99.0 を、いちばん小さい組み合わせで入れる
psbx "rustup toolchain install 1.99.0 --profile minimal" > "$O/u06_minimal_install.txt"
psbx "Set-Location $Q; cargo +1.99.0 fmt --check" > "$O/u06_fmt_missing.txt"
psbx "Set-Location $Q; cargo +1.99.0 clippy" > "$O/u06_clippy_missing.txt"
psbx "rustup component add rustfmt clippy --toolchain 1.99.0" > "$O/u06_component_add.txt"
psbx "Set-Location $Q; cargo +1.99.0 fmt --check" > "$O/u06_fmt_fixed.txt"
psb "rustup toolchain list" > "$O/u06_toolchains_sb.txt"
echo done

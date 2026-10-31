#!/usr/bin/env bash
# 第1巻 UNIT 05 の出力を取る（MSVC Build Tools と CPU の種類）。UNIT 04 と同じ、別のフォルダ（r01\sb）の Rust で試す。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
R=/c/Users/oltot/rsb
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
HELLO='C:\Users\oltot\rsb\w\hello'

# ---- 例5-1: Build Tools が入っているか
psb "& \"\${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe\" -products * -property displayName; & \"\${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe\" -products * -property installationVersion; Get-ChildItem \"\${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\" -Name; Get-ChildItem \"\${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\*\bin\Hostarm64\" -Name" > "$O/u05_vswhere.txt"
psb "rustc -vV" > "$O/u05_rustc_vv.txt"

# ---- 例5-2: 実行ファイルが、どの CPU 用かを読む
cp "$H/u05_machine.ps1" "$R/w/machine.ps1"
psb "Set-Location $HELLO; cargo build -q; & ..\machine.ps1 target\debug\hello.exe" > "$O/u05_host_build.txt"

# ---- エラー: 入っていない target でビルドする
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc" > "$O/u05_target_missing.txt"
# ---- 例5-3: x64 用の部品を足して、x64 用の実行ファイルを作る
psb "rustup target add x86_64-pc-windows-msvc" > "$O/u05_target_add.txt"
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc; & ..\machine.ps1 target\debug\hello.exe target\x86_64-pc-windows-msvc\debug\hello.exe; .\target\x86_64-pc-windows-msvc\debug\hello.exe" > "$O/u05_x64_build.txt"
psb "Set-Location $HELLO; Get-ChildItem target -Directory -Name" > "$O/u05_target_dirs.txt"

# ---- 例5-4: この PC の、ほかの道具は、どの CPU 用か
psb "& C:\Users\oltot\rsb\w\machine.ps1 (Get-Command rustc).Source (Get-Command code).Source ((Get-Command git).Source) 'C:\Program Files\Git\usr\bin\bash.exe' (Get-Command docker).Source \"\$HOME\.fly\bin\fly.exe\"" > "$O/u05_tools_machine.txt"

# ---- エラー: link.exe が見つからない（PATH の中の link.exe を使う設定にして、わざと出す）
psb "Set-Location $HELLO; cargo clean -q; \$env:RUSTFLAGS = '-C linker=link.exe'; cargo build" > "$O/u05_linker_missing.txt"
echo done

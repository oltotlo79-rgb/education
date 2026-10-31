#!/usr/bin/env bash
# 第1巻 UNIT 05 の取り直し（machine.ps1 を直したあと）。target add は済んでいるので、その出力はそのまま使う。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
R=/c/Users/oltot/rsb
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
psb() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:RUSTUP_HOME = 'C:\Users\oltot\rsb\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rsb\cargo'; \$env:Path = 'C:\Users\oltot\rsb\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
HELLO='C:\Users\oltot\rsb\w\hello'
cp "$H/u05_machine.ps1" "$R/w/machine.ps1"
rm -rf "$R/w/hello/target"

# ---- 例5-2: いつものビルドの実行ファイルは、どの CPU 用か
psb "Set-Location $HELLO; cargo build; & ..\machine.ps1 target\debug\hello.exe" > "$O/u05_host_build.txt"
# ---- 例5-3: x64 用の実行ファイルを作る（x86_64-pc-windows-msvc の部品は、u05_target_add で足した）
psb "Set-Location $HELLO; cargo build --target x86_64-pc-windows-msvc; & ..\machine.ps1 target\debug\hello.exe target\x86_64-pc-windows-msvc\debug\hello.exe; .\target\x86_64-pc-windows-msvc\debug\hello.exe" > "$O/u05_x64_build.txt"
psb "Set-Location $HELLO; Get-ChildItem target -Directory -Name" > "$O/u05_target_dirs.txt"
psb "rustup target list --installed" > "$O/u05_target_list.txt"

# ---- 例5-4: この PC の、ほかの道具は、どの CPU 用か
psb "& C:\Users\oltot\rsb\w\machine.ps1 (Get-Command rustc).Source \"\$env:LOCALAPPDATA\Programs\Microsoft VS Code\Code.exe\" (Get-Command git).Source 'C:\Program Files\Git\usr\bin\bash.exe' (Get-Command docker).Source \"\$HOME\.fly\bin\fly.exe\"" > "$O/u05_tools_machine.txt"

# ---- エラー: リンカーが見つからない（この PC には Build Tools があるので、無い名前のリンカーを指定して、同じ状態を作る）
psb "Set-Location $HELLO; \$env:RUSTFLAGS = '-C linker=link-missing.exe'; cargo build" > "$O/u05_linker_missing.txt"
echo done

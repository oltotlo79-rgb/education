#!/usr/bin/env bash
# 第1巻 UNIT 04 の例4-1・4-2（入れた直後の状態）を、新しい置き場 ~\rust-try で取る。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
cd ~ || exit 1
mask() { python "$H/mask01.py"; }
RT=/c/Users/oltot/rust-try
mkdir -p "$RT"; cp ~/.cargo/bin/rustup.exe "$RT/rustup-init.exe"
( cd "$RT" && printf '1\n\n' | RUSTUP_HOME='C:\Users\oltot\rust-try\rustup' CARGO_HOME='C:\Users\oltot\rust-try\cargo' ./rustup-init.exe --no-modify-path > "$O/u04_init.raw" 2>&1 )
mask < "$O/u04_init.raw" > "$O/u04_init.txt"
prt() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location ~; \$env:RUSTUP_HOME = 'C:\Users\oltot\rust-try\rustup'; \$env:CARGO_HOME = 'C:\Users\oltot\rust-try\cargo'; \$env:Path = 'C:\Users\oltot\rust-try\cargo\bin;' + (((\$env:Path -split ';') | Where-Object { \$_ -notmatch 'cargo' }) -join ';'); $1" 2>&1 | mask; }
prt "rustc --version; cargo --version; rustup --version; where.exe rustc cargo rustup" > "$O/u04_versions.txt"
prt "rustup show" > "$O/u04_show.txt"
prt "rustup component list --installed" > "$O/u04_components.txt"
# 例5-4: この PC のいつもの道具（rust-try ではなく、ふだんの PATH）
powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; Set-Location ~; & C:\Users\oltot\rust-study\machine.ps1 (Get-Command rustc).Source \"\$env:LOCALAPPDATA\Programs\Microsoft VS Code\Code.exe\" (Get-Command git).Source 'C:\Program Files\Git\usr\bin\bash.exe' (Get-Command docker).Source \"\$HOME\.fly\bin\fly.exe\"" 2>&1 | mask > "$O/u05_tools_machine.txt"
echo done

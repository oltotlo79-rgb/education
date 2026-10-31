#!/usr/bin/env bash
# 第1巻 UNIT 09 の出力を取る（Turso CLI と libSQL の確認）。Turso のアカウントにはログインしない。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
ps() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:WSL_UTF8 = '1'; $1" 2>&1 | mask; }
wait_url() { for i in $(seq 1 240); do curl.exe -s -o /dev/null "$1" && return 0; sleep 0.5; done; return 1; }

# ---- 例9-1: WSL の Ubuntu に Turso CLI を入れる（公式の手順）
ps "wsl -d Ubuntu -- bash -c 'curl -sSfL https://get.tur.so/install.sh | bash'" > "$O/u09_install.txt"
# 新しいシェル（対話のシェル）で確かめる
ps "wsl -d Ubuntu -- bash -ic 'turso --version; ls ~/.turso; tail -3 ~/.bashrc'" > "$O/u09_version.txt"
# 対話でないシェルでは、PATH の足し算が読まれない
ps "wsl -d Ubuntu -- bash -lc 'turso --version'; 'exit=' + \$LASTEXITCODE" > "$O/u09_noninteractive.txt"
ps "wsl -d Ubuntu -- bash -ic 'turso --help'" > "$O/u09_help.txt"
ps "wsl -d Ubuntu -- bash -ic 'turso dev --help'" > "$O/u09_dev_help.txt"
echo done

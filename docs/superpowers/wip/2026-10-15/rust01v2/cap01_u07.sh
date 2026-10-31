#!/usr/bin/env bash
# 第1巻 UNIT 07 の出力を取る（WSL2 と Docker）。WSL の更新や Docker Desktop の再起動はしない（ほかのコンテナが動いているため）。
set -u
S="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad"
H="$S/rust01"; O="$H/out"
export PYTHONIOENCODING=utf-8
mask() { python "$H/mask01.py"; }
ps() { powershell -NoProfile -Command "[Console]::OutputEncoding = [Text.Encoding]::UTF8; \$env:WSL_UTF8 = '1'; $1" 2>&1 | mask; }

# ---- 例7-1: WSL の版と、入っている Linux
ps "wsl --version" > "$O/u07_wsl_version.txt"
ps "wsl -l -v" > "$O/u07_wsl_list.txt"
ps "wsl -d Ubuntu -- uname -m; wsl -d Ubuntu -- bash -c 'head -4 /etc/os-release; nproc; free -h | head -2'" > "$O/u07_ubuntu.txt"

# ---- 例7-2: Windows と Linux の、ファイルの場所
mkdir -p /c/Users/oltot/rust-study
ps "Set-Location ~\rust-study; wsl -d Ubuntu -- pwd; wsl -d Ubuntu -- bash -c 'cd ~ && pwd && wslpath -w ~'" > "$O/u07_paths.txt"

# ---- 例7-3: 小さなファイルを 2000 個作る時間（Windows 側の /mnt/c と、Linux 側の ~）
ps "Set-Location ~\rust-study; wsl -d Ubuntu -- bash -c 'rm -rf many && mkdir many && cd many && s=\$(date +%s%N) && for i in \$(seq 1 2000); do echo x > f\$i; done && e=\$(date +%s%N) && echo \"/mnt/c (rust-study): \$(( (e-s)/1000000 )) ms\"; cd .. && rm -rf many'; wsl -d Ubuntu -- bash -c 'cd ~ && rm -rf many && mkdir many && cd many && s=\$(date +%s%N) && for i in \$(seq 1 2000); do echo x > f\$i; done && e=\$(date +%s%N) && echo \"Linux home (~): \$(( (e-s)/1000000 )) ms\"; cd ~ && rm -rf many'" > "$O/u07_speed.txt"

# ---- エラー: Linux の名前の間違い
ps "wsl -d Ubunt -- uname -m; 'exit=' + \$LASTEXITCODE" > "$O/u07_wsl_typo.txt"

# ---- 例7-4: Docker
ps "docker version --format 'Client: {{.Client.Version}} {{.Client.Os}}/{{.Client.Arch}}'; docker version --format 'Server: {{.Server.Version}} {{.Server.Os}}/{{.Server.Arch}}'" > "$O/u07_docker_version.txt"
docker rmi -f hello-world > /dev/null 2>&1
ps "docker run --rm hello-world" > "$O/u07_hello_world.txt"
ps "docker run --rm debian:trixie-slim uname -m" > "$O/u07_uname_native.txt"
ps "docker run --rm --platform linux/amd64 debian:trixie-slim uname -m" > "$O/u07_uname_amd64.txt"
ps "docker image ls --format '{{.Repository}}:{{.Tag}} {{.Size}}' hello-world" > "$O/u07_images.txt"

# ---- エラー: ポートが、もう使われている
docker rm -f u07a u07b > /dev/null 2>&1
ps "docker run -d --name u07a -p 18099:80 debian:trixie-slim sleep 60 | Out-Null; docker run -d --name u07b -p 18099:80 debian:trixie-slim sleep 60; 'exit=' + \$LASTEXITCODE" > "$O/u07_port_busy.txt"
docker rm -f u07a u07b > /dev/null 2>&1

# ---- WSL の中から docker を使えるか（Docker Desktop の WSL の連携）
ps "wsl -d Ubuntu -- bash -lc 'command -v docker && docker version --format \"{{.Server.Version}} {{.Server.Os}}/{{.Server.Arch}}\"'" > "$O/u07_wsl_docker.txt"
echo done

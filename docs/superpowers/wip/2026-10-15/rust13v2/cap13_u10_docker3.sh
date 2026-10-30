#!/usr/bin/env bash
# 第13巻 UNIT 10: trixie に替えた教材の Dockerfile で、教材のフォルダからビルドし直す（キャッシュあり）。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
REAL=/c/Users/oltot/Documents/git-projects/def/Rust/rustnote
O="$H/out"
D="$H/u10docker"
export PYTHONIOENCODING=utf-8
docker images --format '{{.Repository}}:{{.Tag}} {{.Size}}' | grep -E '^(rust:1\.99\.1-(bookworm|trixie)|debian:(bookworm|trixie)-slim) ' | sort > "$O/u10_base_sizes.txt"
start=$(date +%s)
docker build --progress=plain -t rustnote:u10 "$REAL" > "$D/build_u10.log" 2>&1
echo "exit=$?・$(( $(date +%s) - start )) 秒" > "$O/u10_build_u10_result.txt"
grep -E "transferring context: [0-9]|^#[0-9]+ \[|CACHED|DONE [0-9.]+s$|naming to" "$D/build_u10.log" | python "$H/mask13.py" | tr -d '\r' | awk '!seen[$0]++' > "$O/u10_build_u10_steps.txt"
{
  docker image inspect rustnote:u10 --format '利用者 {{.Config.User}}・待ち受け {{range $p, $_ := .Config.ExposedPorts}}{{$p}}{{end}}・CMD {{.Config.Cmd}}'
  docker image ls rustnote:u10 --format '大きさ {{.Size}}'
  docker run --rm rustnote:u10 sh -c 'ldd --version | head -1; id; ls -l /app'
  docker run -d --rm --name rn-u10 -p 18080:8080 rustnote:u10 > /dev/null
  for i in $(seq 1 40); do curl.exe -s -o /dev/null "http://127.0.0.1:18080/health" && break; sleep 0.5; done
  curl.exe -s -i "http://127.0.0.1:18080/health" | tr -d '\r' | grep -E "^(HTTP|content-type)|status"
  echo "--- コンテナのログ"
  docker logs rn-u10 2>&1 | tr -d '\r'
  docker stop rn-u10 > /dev/null
} > "$O/u10_run_u10.txt" 2>&1
echo done

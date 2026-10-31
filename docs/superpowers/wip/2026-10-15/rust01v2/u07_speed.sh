#!/usr/bin/env bash
# 小さなファイルを 2000 個作る時間を、Windows 側のフォルダと、Linux 側のフォルダで比べる
count() {
  rm -rf "$1/many" && mkdir -p "$1/many" && cd "$1/many" || return
  s=$(date +%s%N)
  for i in $(seq 1 2000); do echo x > "f$i"; done
  e=$(date +%s%N)
  echo "$2: $(( (e - s) / 1000000 )) ms"
  cd / && rm -rf "$1/many"
}
count "$PWD" "Windows 側（$PWD）"
count "$HOME" "Linux 側（$HOME）"

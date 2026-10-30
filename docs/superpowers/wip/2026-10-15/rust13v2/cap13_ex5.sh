#!/usr/bin/env bash
# 第13巻 総合演習5: nav に名前を付け、狭い画面のヘッダーを折り返す。fixed の release を Chrome で測る。
set -u
H="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust13"
O="$H/out"
(cd "$H/fixed" && CARGO_TARGET_DIR="$H/target_fixed" topcoat asset bundle --release > /dev/null 2>&1) || exit 1
RUN="$H/ex5run"; rm -rf "$RUN"; mkdir -p "$RUN"
( cd "$RUN" && PORT=3014 DATABASE_URL=ex5.db "$H/target_fixed/release/rustnote.exe" > "$RUN/server.log" 2>&1 & )
for i in $(seq 1 40); do sleep 0.25; curl.exe -s -o /dev/null http://127.0.0.1:3014/health && break; done
(cd "$H/u09lab" && node measure.mjs all 2>&1 | tr -d '\r') > "$O/ex5_measure.txt"
taskkill //F //IM rustnote.exe > /dev/null 2>&1
sleep 1
rm -rf "$RUN"
echo done

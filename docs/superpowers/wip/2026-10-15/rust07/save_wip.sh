#!/usr/bin/env bash
# 7巻の下書き（parts/）と追加した素材を作業ブランチに保存してプッシュする。使い方: save_wip.sh "<メッセージ>"
set -euo pipefail
S=/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust07
REPO=/c/Users/oltot/Documents/git-projects/def
D=$REPO/docs/superpowers/wip/2026-10-15/rust07
cd $S
for dir in parts sql-lab/sql sql-lab/src out2 out8 out9 core-only/src; do
  [ -d "$dir" ] || continue
  find "$dir" -type f ! -name "*.db" | while read f; do mkdir -p "$D/$(dirname "$f")"; cp "$f" "$D/$f"; done
done
cp sql-lab/Cargo.toml sql-lab/Cargo.lock "$D/sql-lab/"
cp core-only/Cargo.toml "$D/core-only/"
cp assemble.py save_wip.sh "$D/"
git -C $REPO add docs/superpowers/wip/2026-10-15/rust07
if ! git -C $REPO diff --cached --quiet; then
  git -C $REPO commit -q -m "wip: $1

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
  git -C $REPO push -q origin HEAD
fi
git -C $REPO log --oneline -1

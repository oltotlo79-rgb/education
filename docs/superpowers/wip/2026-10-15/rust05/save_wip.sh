#!/usr/bin/env bash
# 5巻の下書き（parts/）と追加した素材を作業ブランチに保存してプッシュする。使い方: save_wip.sh "<メッセージ>"
set -euo pipefail
S=/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust05
REPO=/c/Users/oltot/Documents/git-projects/def
D=$REPO/docs/superpowers/wip/2026-10-15/rust05
cd $S
for dir in parts async_lab/src errs out out01 out02 out03 out04 out05 out06 out07 out08 out09 out10; do
  [ -d "$dir" ] || continue
  find "$dir" -type f | while read f; do mkdir -p "$D/$(dirname "$f")"; cp "$f" "$D/$f"; done
done
mkdir -p "$D/probe"
cp probe/*.txt "$D/probe/" 2>/dev/null || true
cp async_lab/Cargo.toml async_lab/Cargo.lock async_lab/rust-toolchain.toml "$D/async_lab/"
cp assemble.py save_wip.sh mask.py cap.sh errcap.sh render.py "$D/"
git -C $REPO add docs/superpowers/wip/2026-10-15/rust05
if ! git -C $REPO diff --cached --quiet; then
  git -C $REPO commit -q -m "wip: $1

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
  git -C $REPO push -q origin HEAD
fi
git -C $REPO log --oneline -1

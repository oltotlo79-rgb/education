#!/usr/bin/env bash
# 完成したファイルを main（worktree）へコミット・プッシュし、作業用ブランチにも同じ内容をコミット・プッシュする。
# 使い方: publish.sh "<コミットメッセージ1行目>" "<本文（複数行可）>" <リポジトリ相対パス>...
#   パスが作業ツリーに無い場合は「削除」として扱う（旧 Rust/*.pdf の置き換え用）。
set -euo pipefail
REPO="C:/Users/oltot/Documents/git-projects/def"
MAINWT="C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/main-wt"
SUBJECT="$1"; BODY="$2"; shift 2
MSG="$SUBJECT

$BODY

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"

git -C "$MAINWT" pull -q --ff-only origin main
for f in "$@"; do
  if [ -e "$REPO/$f" ]; then
    mkdir -p "$(dirname "$MAINWT/$f")"
    cp -r "$REPO/$f" "$MAINWT/$f"
    git -C "$MAINWT" add -- "$f"
  else
    git -C "$MAINWT" rm -q -r --ignore-unmatch -- "$f"
  fi
done
git -C "$MAINWT" commit -q -m "$MSG"
git -C "$MAINWT" push -q origin main
echo "main: $(git -C "$MAINWT" log --oneline -1)"

for f in "$@"; do
  if [ -e "$REPO/$f" ]; then
    git -C "$REPO" add -- "$f"
  else
    git -C "$REPO" rm -q -r --cached --ignore-unmatch -- "$f"
  fi
done
if ! git -C "$REPO" diff --cached --quiet; then
  git -C "$REPO" commit -q -m "wip: ${SUBJECT}（mainと同じ内容）

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
  git -C "$REPO" push -q origin HEAD
fi
echo "wip:  $(git -C "$REPO" log --oneline -1)"

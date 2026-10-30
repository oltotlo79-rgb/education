#!/usr/bin/env bash
# 1つの巻を組み立てて確かめる: fill → assemble → check_md → PDF → 生の Markdown・メールの検査 → 新しい図の画像
# 使い方: bash build_vol.sh <巻のフォルダ名（rust06 など）> <UNIT 番号> <Rust/ の下の md のファイル名>
set -u
S="$(cd "$(dirname "$0")" && pwd)"
VOLDIR="$S/$1"; N=$(printf "%02d" "$2"); MD="$3"
REPO="C:/Users/oltot/Documents/git-projects/def"
export PYTHONIOENCODING=utf-8
cd "$VOLDIR" || exit 1
if grep -n "64748b:margin" parts/unit$N.src.md; then echo "図の説明文の style の書き間違い"; exit 1; fi
python - "parts/unit$N.src.md" <<'PY' || exit 1
import re, sys
s = open(sys.argv[1], encoding="utf-8").read()
bs = chr(92)
bad = [t for t in re.findall(r"<text[^>]*>[^<]*</text>", s) if bs in t and "Consolas" not in t]
bad += [t for t in re.findall(r'<div style="font-size:12px;color:#64748b;[^"]*">[^\n]*', s) if bs in t and "Consolas" not in t]
for t in bad:
    print("¥ に見える \\ :", t[:100])
sys.exit(1 if bad else 0)
PY
python fill.py "unit$N" | tail -1 || exit 1
python assemble.py | tail -2
cd "$REPO" || exit 1
F="Rust/$MD"
python docs/superpowers/wip/2026-10-15/ref/check_md.py "$F" | tail -1
ME="$(git -C "$REPO" config user.email | cut -d@ -f1)"
if [ -n "$ME" ] && grep -q "$ME" "$F"; then echo "メールアドレスが入っている"; exit 1; fi
python -c "import sys; b=open(sys.argv[1],'rb').read(); print('CR:', b.count(b'\r'))" "$F"
python _build_pdfs.py "$F" | tail -1
P="Rust/pdf/${MD%.md}.pdf"
python - "$P" "$VOLDIR/parts/unit$N.md" <<'PY'
import re, sys, fitz
d = fitz.open(sys.argv[1])
unit = open(sys.argv[2], encoding="utf-8").read()
# この UNIT の図の番号（assemble 後の md から拾う）
print("pages", d.page_count)
for i, pg in enumerate(d):
    t = pg.get_text()
    for m in ("```", "|---|", "@@", "<!--", "¥"):
        if m in t:
            print("  check page", i + 1, repr(m))
PY

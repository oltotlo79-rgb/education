import subprocess
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
MD = Path(r"C:/Users/oltot/Documents/git-projects/def/next/06-project-setup.md")
HEAD = '<div style="max-width: 720px; margin: 18px auto; text-align: center;">'

PAIRS = [
    ("figA", "最初の質問の画面イメージ"),
    ("figB", "質問に答え終わった画面イメージ"),
    ("figC", "推奨設定と本書の設定の違い（"),
    ("figD", "create-next-app が作ったファイルと、それぞれの役割"),
    ("alias_fig", "パスエイリアス @/ が実際のファイルにたどり着くまで"),
    ("figE", ".env.local の値がコードに届くまで"),
    ("figF", "npm run dev を実行したときのターミナル"),
    ("figG", "を開いたときの画面イメージ"),
    ("types_fig", "Book / BookInsert / BookUpdate に入る項目の比較"),
]
only = set(sys.argv[1:])
s = MD.read_text(encoding="utf-8")
for name, key in PAIRS:
    if only and name not in only:
        continue
    cnt = s.count(key)
    if cnt != 1:
        print("SKIP (count", cnt, ")", name)
        continue
    k = s.index(key)
    start = s.rindex(HEAD, 0, k)
    end = s.index("</div>\n</div>", k) + len("</div>\n</div>")
    new = (BASE / "gen" / f"{name}.html").read_text(encoding="utf-8").rstrip("\n")
    s = s[:start] + new + s[end:]
    print("replaced", name)
MD.write_text(s, encoding="utf-8")

"""md 内に埋め込み済みの図を、gen/<name>.html の最新版に差し替える。
使い方: python refig.py name:キャプションの一部 [...]
"""
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
MD = Path(r"C:/Users/oltot/Documents/git-projects/def/next/06-project-setup.md")
HEAD = '<div style="max-width: 720px; margin: 18px auto; text-align: center;">'

s = MD.read_text(encoding="utf-8")
for arg in sys.argv[1:]:
    name, key = arg.split(":", 1)
    assert s.count(key) == 1, f"caption key not unique ({s.count(key)}): {key}"
    k = s.index(key)
    start = s.rindex(HEAD, 0, k)
    end = s.index("</div>\n</div>", k) + len("</div>\n</div>")
    new = (BASE / "gen" / f"{name}.html").read_text(encoding="utf-8").rstrip("\n")
    s = s[:start] + new + s[end:]
    print("replaced", name)
MD.write_text(s, encoding="utf-8")

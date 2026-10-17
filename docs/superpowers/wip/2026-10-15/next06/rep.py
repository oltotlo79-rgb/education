"""Part ファイルの指示どおりに 06-project-setup.md の一部を置き換える。

part ファイルの先頭行:
  @@START: <開始文字列>              置き換え開始位置（この文字列から）
  @@END: <終了文字列>                置き換え終了位置（この文字列の直前まで）
  @@ENDINC: <終了文字列>             置き換え終了位置（この文字列を含む）
  @@AFTER: <文字列>                  この文字列の直後に挿入（START/END の代わり）
  @@BEFORE: <文字列>                 この文字列の直前に挿入
本文中の行 FIG:<name> は gen/<name>.html の中身に置き換える。
"""
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent
MD = Path(r"C:/Users/oltot/Documents/git-projects/def/next/06-project-setup.md")


def main():
    for part in sys.argv[1:]:
        lines = Path(part).read_text(encoding="utf-8").split("\n")
        opts = {}
        while lines and lines[0].startswith("@@"):
            k, v = lines.pop(0)[2:].split(": ", 1)
            opts[k] = v.replace("\\n", "\n")
        body_lines = []
        for l in lines:
            if l.startswith("CODE:"):
                body_lines.append((BASE / "parts" / l[5:].strip()).read_text(encoding="utf-8").rstrip(chr(10)))
            elif l.startswith("FIG:"):
                body_lines.append((BASE / "gen" / (l[4:].strip() + ".html")).read_text(encoding="utf-8").rstrip("\n"))
            else:
                body_lines.append(l)
        body = "\n".join(body_lines)
        s = MD.read_text(encoding="utf-8")
        if "AFTER" in opts:
            a = opts["AFTER"]
            assert s.count(a) == 1, f"AFTER not unique/found ({s.count(a)}): {a[:40]}"
            i = s.index(a) + len(a)
            s = s[:i] + body + s[i:]
        elif "BEFORE" in opts:
            a = opts["BEFORE"]
            assert s.count(a) == 1, f"BEFORE not unique/found ({s.count(a)}): {a[:40]}"
            i = s.index(a)
            s = s[:i] + body + s[i:]
        else:
            st = opts["START"]
            assert s.count(st) == 1, f"START not unique/found ({s.count(st)}): {st[:40]}"
            i = s.index(st)
            if "END" in opts:
                j = s.index(opts["END"], i)
            else:
                j = s.index(opts["ENDINC"], i) + len(opts["ENDINC"])
            s = s[:i] + body + s[j:]
        MD.write_text(s, encoding="utf-8")
        print("applied", Path(part).name)


main()

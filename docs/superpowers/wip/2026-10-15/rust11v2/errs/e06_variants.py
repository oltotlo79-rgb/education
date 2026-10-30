"""第11巻 UNIT 06: 守りを1つずつ外した版の一覧。
（名前, 書き換えるファイル, 置き換え表）。置き換え表は、第8巻・第11巻 UNIT 02 で使ったものを読み込む。
使い方: python e06_variants.py <名前>   → その版の置き換え表のパスを、標準出力に書く（edit_src.py に渡す）
"""
import os
import runpy
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
R08 = os.path.join(HERE, "..", "..", "rust08", "errs")

VARIANTS = {
    # 名前: (ファイル, 表を読む関数)
    "toggle": ("src/repository/notes.rs", lambda: runpy.run_path(os.path.join(R08, "e07_tables.py"))["TOGGLE"]),
    "delete": ("src/repository/notes.rs", lambda: runpy.run_path(os.path.join(HERE, "e02_tables.py"))["NO_OWNER"]),
    "search": ("src/repository/notes.rs", lambda: runpy.run_path(os.path.join(R08, "e07_tables.py"))["PRECEDENCE"]),
    "origin": ("src/routes.rs", lambda: runpy.run_path(os.path.join(R08, "e06_tables.py"))["DISABLE"]),
    "logout": ("src/auth.rs", lambda: runpy.run_path(os.path.join(R08, "e05_logout.py"))["REPLACEMENTS"]),
}

if __name__ == "__main__":
    name = sys.argv[1]
    path, table = VARIANTS[name]
    out = os.path.join(HERE, f"e06_variant_{name}.py")
    with open(out, "w", encoding="utf-8", newline="\n") as f:
        f.write(f"REPLACEMENTS = {table()!r}\n")
    print(path, out)

"""練習用コピーの src のファイルを、1か所だけ書き換える（見つからなければ止まる）。
使い方: python edit_src.py <ファイル> <置き換え表の .py>
置き換え表の .py には、REPLACEMENTS = [(古い文字列, 新しい文字列), ...] を書く。
"""
import runpy
import sys

path, table = sys.argv[1], sys.argv[2]
replacements = runpy.run_path(table)["REPLACEMENTS"]
text = open(path, encoding="utf-8", newline="").read()
for old, new in replacements:
    if text.count(old) != 1:
        sys.exit(f"{path}: 1か所だけ見つかるはずが {text.count(old)} か所: {old[:60]!r}")
    text = text.replace(old, new)
open(path, "w", encoding="utf-8", newline="").write(text)
print(f"{path}: {len(replacements)} か所を書き換えた")

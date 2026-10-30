from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit04.src.md"
s = p.read_text(encoding="utf-8")
reps = [
    (
        "の文字列でしか表に出ないので、20 文字より短くは縮められません。`successes: 0` は、1つ目の入力で、もう失敗したという意味です。",
        "の文字列でしか表に出ないので、21 文字より短くは縮められません（1文字は多くても 4 バイトなので、20 文字では 80 バイトを超えません）。`successes: 1` は、1つ通ったあとの、2つ目の入力で、もう失敗したという意味です。",
    ),
    (
        "- 2回目は、保存された種から、**同じ入力**が最初に試されて、同じように失敗しました。",
        "- 2回目は、`successes: 0` のとおり、保存された種から、**同じ入力**が最初に試されて、同じように失敗しました。",
    ),
]
for old, new in reps:
    assert s.count(old) == 1, old[:70]
    s = s.replace(old, new)
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")

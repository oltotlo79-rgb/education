from pathlib import Path

p = Path(__file__).resolve().parent / "parts" / "unit04.src.md"
s = p.read_text(encoding="utf-8")
reps = [
    (
        r"@@OUTGREP:u04_spaces2:^(minimal|\tsuccesses)@@",
        r"@@OUTGREP:u04_spaces1:^(minimal|\tsuccesses)@@",
    ),
    (
        "- 3回とも、6〜9 個目の入力で見つかりました（上は2回目。5 個通ったあとの、6 個目で失敗）。2回目は、空白1つの `\" \"` まで縮みました。1回目と3回目は、81 文字くらいの、空白の多い文字列で止まりました。「空白だけ」と「80 文字の境目」のどちらで見つけたかで、縮み方が変わります。",
        "- 3回とも見つかりました。`successes` は 9・0・37 で、10 個目・1 個目・38 個目の入力で失敗しました（上は1回目）。3回とも、81 文字（前後の空白を除くと 78〜80 文字）の文字列まで縮みました。`forgot_trim` は 81 文字なので不合格、`rule` は前後の空白を除くと 80 文字以下なので合格、という「80 文字の境目」での食い違いです。\n- この作り方で試した別の回には、空白1つの `\" \"` や、タブ1つの `\"\\t\"`（空白だけの食い違い）まで縮んだこともありました。どちらの食い違いを先に見つけたかで、縮み方が変わります。",
    ),
]
for old, new in reps:
    assert s.count(old) == 1, old[:70]
    s = s.replace(old, new)
p.write_text(s, encoding="utf-8", newline="\n")
print("ok")

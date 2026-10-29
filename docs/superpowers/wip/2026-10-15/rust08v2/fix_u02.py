import sys
p = "parts/unit02.src.md"
s = open(p, encoding="utf-8").read()
reps = [
 ("color:#64748b:margin-top", "color:#64748b;margin-top"),
 ("1秒に 39 回（棒の長さは、速いハッシュの 22 万分の 1 で、ほとんど見えない）", "1秒に 39 回（本当の棒の長さは上の 22 万分の 1 で、描けないほど短い）"),
 ("古いハッシュは、その人が次にログインしたときに、新しい設定で作り直せばよいのです（UNIT 04（準備中））。", "古いハッシュは、その人が次にログインして照合に成功したときに、新しい設定で作り直せばよいのです。"),
 ("- 2回目と3回目は、どちらも約 380〜410 ミリ秒（debug ビルド）で、そろっています。", "- 「登録していないメール」の2回目・3回目と、「パスワードが違う」の3回は、どれも約 380〜410 ミリ秒（debug ビルド）で、そろっています。"),
 ("`SaltString` は無くなりました。", "`SaltString` は、この場所（`password_hash` の直下）にはありません。"),
]
for a, b in reps:
    n = s.count(a)
    if n != 1:
        sys.exit(f"count {n}: {a[:40]}")
    s = s.replace(a, b)
open(p, "w", encoding="utf-8").write(s)
print("ok")

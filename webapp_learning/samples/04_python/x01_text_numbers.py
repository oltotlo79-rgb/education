# 付録 B-11: 文字列と数でよく使う書き方
title = "  治具の製作  "
requester = "山田 太郎"
amount = 1234567.891

# f 文字列: f"..." の { } に値や式を書ける。: のあとで形を決められる
print(f"依頼者: {requester}、件名: {title.strip()}")
print(f"{amount:,.2f}")   # 3 けたごとの区切り ＋ 小数点以下 2 けた
print(f"{42:05d}")        # 5 けたになるまで前を 0 で埋める
print(f"[{'右':>6}] [{'左':<6}] [{'中':^6}]")  # 幅 6 で右・左・中央に寄せる
print(f"{0.256:.1%}")     # 割合（% の形）

# よく使う文字列の操作
print("[" + title.strip() + "]")          # 前後の空白を取る
print(len(title.strip()))                 # 文字の数
print("a,b,c".split(","))                 # 区切ってリストに
print("、".join(["山田", "佐藤"]))          # リストをつないで 1 つに（区切り文字の方に .join）
print("2026-11-02".replace("-", "/"))     # 置きかえる
print("approved".upper(), "APPROVED".lower())
print("依頼書.pdf".endswith(".pdf"), "依頼" in "依頼書.pdf")  # 終わる？ 含む？
print("REQ-0001"[4:], "REQ-0001"[:3], "REQ-0001"[-1])     # 切り出し（0 から数える。-1 は最後）
print("7".zfill(4))                       # 前を 0 で埋めて 4 けたに

# 文字列 → 数: int() / float()。数でない時は ValueError
print(int("42") + 1, float("3.5") * 2)
try:
    int("42円")
except ValueError as e:
    print("数にできない:", e)

# 数の計算
print(7 / 2, 7 // 2, 7 % 2, 2 ** 10)      # 割り算・切り捨ての割り算・余り・累乗
print(round(2.5), round(3.5), round(2.675, 2))  # round は「偶数への丸め」と誤差に注意
print(max(3, 9, 4), min([5, 1, 8]), sum([1, 2, 3]), abs(-5))
print(0.1 + 0.2, 0.1 + 0.2 == 0.3)        # 小数の誤差（お金は「円」の整数で持つ）

# 付録 B-12: リスト・辞書・集合・タプルでよく使う書き方
from collections import Counter, defaultdict

requests = [
    {"id": 1, "title": "治具の製作", "status": "回覧中", "amount": 30000},
    {"id": 2, "title": "部品の追加工", "status": "承認済み", "amount": 12000},
    {"id": 3, "title": "棚の取り付け", "status": "差し戻し", "amount": 8000},
    {"id": 4, "title": "工具の修理", "status": "回覧中", "amount": 5000},
]

# リスト: 足す・取り出す・並べかえる
names = ["山田"]
names.append("佐藤")          # 後ろに足す
names.insert(0, "鈴木")       # 0 番目に入れる
last = names.pop()            # 後ろから取り出す
print(names, last, len(names))

# sorted は「新しい」リストを返す。key で「何で並べるか」を決める。reverse=True で逆順
by_amount = sorted(requests, key=lambda r: r["amount"])
print([r["title"] for r in by_amount])
print([r["id"] for r in sorted(requests, key=lambda r: r["amount"], reverse=True)])

# 内包表記: for と if を 1 行で書いて、新しいリストを作る
circulating = [r["id"] for r in requests if r["status"] == "回覧中"]
print("回覧中:", circulating)
print("合計:", sum(r["amount"] for r in requests))
print("2 万円より高い物がある？", any(r["amount"] > 20000 for r in requests))

# enumerate: 番号つきで回す（start=1 で 1 から）。zip: 2 つを組にして回す
for i, r in enumerate(requests[:2], start=1):
    print(f"{i}件目: {r['title']}")
for order, name in zip([1, 2], ["佐藤", "鈴木"]):
    print(f"{order}番目の承認者: {name}")

# 辞書: get は、キーが無い時の値を決められる（[ ] だと KeyError）
user = {"employee_no": "012345", "name": "山田 太郎"}
print(user.get("mail", "（メール無し）"))
user["dept"] = "設計部"                     # 足す・書きかえる
for key, value in user.items():             # キーと値を組で回す
    print(f"{key} = {value}")
print(list(user.keys()), "name" in user)

# 辞書の内包表記: id → 件名 の表を作る
titles = {r["id"]: r["title"] for r in requests}
print(titles[3])

# Counter: 数え上げ。most_common で多い順
counts = Counter(r["status"] for r in requests)
print(counts, counts.most_common(1))

# defaultdict: 無いキーを初めて使う時、自動で空のリストを作る（グループ分け）
by_status = defaultdict(list)
for r in requests:
    by_status[r["status"]].append(r["id"])
print(dict(by_status))

# 集合（set）: 重複を消す・共通・差
a = {"設計", "製造", "品質"}
b = {"製造", "総務"}
print(sorted(a | b), sorted(a & b), sorted(a - b))
print(sorted(set(["設計", "製造", "設計"])))

# タプル: 変えられない組。複数の値を返す・受け取る時によく使う
point = (10, 20)
x, y = point
print(x + y)

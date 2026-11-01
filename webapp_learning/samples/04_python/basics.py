# 1. 変数と型（型ヒントは「説明」で、Python は動かすときに確かめない）
app_name: str = "依頼・承認アプリ"
count: int = 3
price: float = 1234.5
is_open: bool = True
count = count + 1

# 2. リスト（順番のある並び）
approvers: list[str] = ["山田", "佐藤", "鈴木"]
approvers.append("高橋")              # 後ろに足す

# 3. 辞書（名前と値の組の集まり）
user: dict[str, str] = {"employee_no": "012345", "name": "山田 太郎"}
user["dept"] = "設計部"               # 足す・書きかえる


# 4. 関数（def で作る。中身は字下げで表す）
def greet(name: str) -> str:
    return f"{name} さん、こんにちは"   # f"..." の { } に値を埋め込める


# 5. if と for
for i, name in enumerate(approvers, start=1):
    if i == 1:
        print(f"{i} 番目: {name}（最初）")
    elif i == len(approvers):
        print(f"{i} 番目: {name}（最後）")
    else:
        print(f"{i} 番目: {name}")

print(app_name, count, price, is_open)
print(user["name"], user.get("mail", "（メール無し）"))  # get は、無いときの値を決められる
print(greet(user["name"]))

# 6. 内包表記（リストを作る短い書き方）
lengths = [len(n) for n in approvers]
print(lengths)

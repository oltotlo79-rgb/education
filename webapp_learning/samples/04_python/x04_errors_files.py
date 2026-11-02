# 付録 B-14: エラー・ファイル・JSON・CSV でよく使う書き方
import csv
import json
import shutil
from pathlib import Path


class TitleError(ValueError):
    """件名の間違い（自分で作るエラーの種類）"""


def check_title(title: str) -> str:
    if not title.strip():
        raise TitleError("件名を入れてください")   # raise でエラーを起こす
    return title.strip()


# try / except / else / finally
for t in ["  治具の製作 ", "   "]:
    try:
        ok = check_title(t)
    except TitleError as e:
        print("入力の間違い:", e)
    else:
        print("OK:", ok)              # エラーが起きなかった時だけ
    finally:
        print("（毎回ここを通る）")

# Path: ファイルとフォルダの場所を扱う。/ でつなげる
work = Path("work")
data = work / "data"
data.mkdir(parents=True, exist_ok=True)       # 途中のフォルダもまとめて作る

# 書く・読む（日本語を扱う時は encoding="utf-8" を必ず付ける）
memo = data / "memo.txt"
memo.write_text("1 行目\n2 行目\n", encoding="utf-8")
with memo.open("a", encoding="utf-8") as f:   # with: 使い終わると自動で閉じる
    f.write("3 行目（書き足し）\n")
for i, line in enumerate(memo.read_text(encoding="utf-8").splitlines(), start=1):
    print(f"{i}: {line}")
print(memo.exists(), memo.name, memo.suffix, memo.stat().st_size, "バイト")

# JSON: ensure_ascii=False で日本語をそのまま、indent=2 で読みやすく
req = {"id": 1, "title": "治具の製作", "approvers": ["佐藤", "鈴木"]}
(data / "req.json").write_text(json.dumps(req, ensure_ascii=False, indent=2), encoding="utf-8")
back = json.loads((data / "req.json").read_text(encoding="utf-8"))
print(back["approvers"][1], json.dumps(back, ensure_ascii=False))

# CSV: DictWriter で書いて、DictReader で読む（Excel で開くなら encoding="utf-8-sig"）
rows = [{"no": "012345", "name": "山田 太郎"}, {"no": "012346", "name": "佐藤 花子"}]
with (data / "users.csv").open("w", encoding="utf-8-sig", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=["no", "name"])
    writer.writeheader()
    writer.writerows(rows)
with (data / "users.csv").open(encoding="utf-8-sig", newline="") as f:
    for row in csv.DictReader(f):
        print(row["no"], row["name"])

# フォルダの中の一覧（並べかえてから）
print(sorted(p.name for p in data.iterdir()))

# 無いファイルを読むと FileNotFoundError
try:
    (data / "none.txt").read_text(encoding="utf-8")
except FileNotFoundError:
    print("ファイルがありません")

shutil.rmtree(work)                               # 片づけ: フォルダごと消す（よく確かめて使う）
print("work は残っている？", work.exists())

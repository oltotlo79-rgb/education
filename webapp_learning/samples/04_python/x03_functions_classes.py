# 付録 B-13: 関数とクラスでよく使う書き方
from dataclasses import dataclass, field
from enum import StrEnum


# 既定値つきの引数・名前付きで渡す引数
def make_title(name: str, kind: str = "依頼", urgent: bool = False) -> str:
    mark = "【至急】" if urgent else ""
    return f"{mark}{name}の{kind}"


print(make_title("治具"))
print(make_title("治具", "見積もり"))
print(make_title("治具", urgent=True))      # 名前を付けて渡すと、順番を気にしなくてよい


# *args: いくつでも受け取る（タプルになる）。**kwargs: 名前付きでいくつでも（辞書になる）
def total(*amounts: int, **options: str) -> str:
    unit = options.get("unit", "円")
    return f"{sum(amounts):,}{unit}"


print(total(1000, 2500, 300))
print(total(5, 7, unit="個"))


# 複数の値を返す（本当はタプルを 1 つ返している）
def min_max(values: list[int]) -> tuple[int, int]:
    return min(values), max(values)


low, high = min_max([30000, 12000, 8000])
print(low, high)


# StrEnum: 決まった値の一覧（文字列としても使える）
class Status(StrEnum):
    CIRCULATING = "回覧中"
    APPROVED = "承認済み"
    RETURNED = "差し戻し"


# クラス: データと、それを扱う関数（メソッド）をまとめる
class Request:
    def __init__(self, title: str, amount: int):   # 作る時に呼ばれる
        self.title = title
        self.amount = amount
        self.status = Status.CIRCULATING

    def approve(self) -> None:                      # self は「自分」
        self.status = Status.APPROVED

    @property                                       # () 無しで値のように読める
    def is_expensive(self) -> bool:
        return self.amount >= 20000

    def __str__(self) -> str:                       # print した時の形
        return f"{self.title}（{self.amount:,}円・{self.status}）"


r = Request("治具の製作", 30000)
print(r, r.is_expensive)
r.approve()
print(r, r.status == Status.APPROVED, r.status == "承認済み")


# @dataclass: 項目を書くだけで、__init__・__repr__・== を自動で作る
@dataclass
class Approval:
    order: int
    approver: str
    comments: list[str] = field(default_factory=list)  # リストの既定値はこう書く
    done: bool = False


a1 = Approval(1, "佐藤")
a2 = Approval(1, "佐藤")
a1.comments.append("確認しました")
print(a1)
print(a1 == a2, Approval(2, "鈴木", done=True))

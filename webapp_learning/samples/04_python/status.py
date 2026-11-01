"""依頼の状態と、状態の移り変わり（02 TypeScript の status.ts を Python で書いた物）"""

from dataclasses import dataclass, field
from enum import StrEnum


class Status(StrEnum):
    """依頼の状態。この 7 つ以外は作れない"""

    CIRCULATING = "回覧中"
    RETURNED = "差し戻し中"
    PULLED_BACK = "引き戻し中"
    APPROVED = "回覧完了"
    IN_PROGRESS = "対応中"
    DONE = "完了"
    WITHDRAWN = "取り下げ"


# (今の状態, 操作) → 次の状態 の表。表に無い組み合わせは、できない操作
TRANSITIONS: dict[tuple[Status, str], Status] = {
    (Status.CIRCULATING, "全員が承認"): Status.APPROVED,
    (Status.CIRCULATING, "差し戻し"): Status.RETURNED,
    (Status.CIRCULATING, "引き戻し"): Status.PULLED_BACK,
    (Status.RETURNED, "再依頼"): Status.CIRCULATING,
    (Status.RETURNED, "取り下げ"): Status.WITHDRAWN,
    (Status.PULLED_BACK, "再依頼"): Status.CIRCULATING,
    (Status.PULLED_BACK, "取り下げ"): Status.WITHDRAWN,
    (Status.APPROVED, "担当を決める"): Status.IN_PROGRESS,
    (Status.IN_PROGRESS, "引き渡し"): Status.DONE,
}


class TransitionError(Exception):
    """できない操作をしようとしたときのエラー"""


@dataclass
class Request:
    """依頼 1 件"""

    id: int
    title: str
    status: Status = Status.CIRCULATING
    comments: list[str] = field(default_factory=list)

    def apply(self, action: str) -> None:
        """操作をして、状態を進める。できない操作なら TransitionError を起こす"""
        key = (self.status, action)
        if key not in TRANSITIONS:
            raise TransitionError(f"「{self.status}」の依頼に「{action}」はできません")
        self.status = TRANSITIONS[key]


if __name__ == "__main__":  # このファイルを直接動かしたときだけ、下を動かす
    r = Request(id=1, title="治具の製作")
    print(r)
    r.apply("差し戻し")
    r.comments.append("【10/21 10:15 回覧者1 山田】寸法を追記してください")
    print(r.status, r.comments)
    r.apply("再依頼")
    print(r.status)
    try:
        r.apply("引き渡し")
    except TransitionError as e:
        print("エラー:", e)
    print([s.value for s in Status])

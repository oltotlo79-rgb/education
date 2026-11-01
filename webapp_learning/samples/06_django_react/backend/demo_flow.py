"""前編の練習: サーバーの操作だけで、依頼を出して回覧を進める。
使い方: .\\dev.ps1 shell -c "import demo_flow"
（メールは sent_mail フォルダにファイルで書かれる。中の URL をブラウザで開いて、承認の画面を試す）
"""

from datetime import date

from accounts.models import User
from workflow import services
from workflow.models import Request

yamada = User.objects.get(employee_no="012345")
sato = User.objects.get(employee_no="012346")
suzuki = User.objects.get(employee_no="012347")

r = services.submit(
    yamada,
    title="検査用の治具の製作",
    detail="部品 A-102 の穴の位置を確かめる治具。\n数量は 2 台。",
    due_date=date(2026, 11, 30),
    approver_ids=[sato.pk, suzuki.pk],
    drawing={"version": 1, "widthMm": 297, "heightMm": 210, "shapes": [
        {"kind": "rect", "x": 20, "y": 20, "width": 120, "height": 70},
        {"kind": "rect", "x": 160, "y": 40, "width": 90, "height": 120},
    ]},
)
print(f"依頼 {r.pk} を出しました。状態: {r.get_status_display()}")
print("回覧:", [(s.order, s.approver.name, s.get_decision_display()) for s in r.steps.all()])
print("記録:", [h.action for h in r.histories.all()])
print("Request の件数:", Request.objects.count())

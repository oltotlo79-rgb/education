"""後編の練習: グラフを見るために、いろいろな状態の依頼を足す。
使い方: .\\dev.ps1 shell -c "import demo_more"
"""

from datetime import date

from django.db import transaction

from accounts.models import User
from workflow import services
from workflow.models import Step

yamada, sato, suzuki = (User.objects.get(employee_no=n) for n in ["012345", "012346", "012347"])
takahashi = User.objects.get(employee_no="020001")
ito, _ = User.objects.update_or_create(
    employee_no="020002",
    defaults={"name": "伊藤 健", "dept": "設備課", "job_title": "主任", "role": User.Role.WORKER, "email": "020002@example.com"},
)


def approve_all(r):
    """練習用: メールの URL の代わりに、回覧を全部承認する（本物の画面では、承認者が URL から行う）"""
    for step in Step.objects.filter(request=r, round=r.round).order_by("order"):
        token = services.secrets.token_urlsafe(32)
        with transaction.atomic():
            step.token_hash = services._hash(token)
            step.save(update_fields=["token_hash"])
        services.approve(token)


titles = ["棚の取り付け", "型の改造", "部品の追加工", "台車の修理", "工具箱の作成", "架台の溶接"]
made = []
for i, title in enumerate(titles):
    r = services.submit(yamada if i % 2 == 0 else sato, title=title, detail="", due_date=date(2026, 12, 1 + i),
                        approver_ids=[suzuki.pk])
    made.append(r)

approve_all(made[0]); services.assign(takahashi, made[0].pk); services.complete(takahashi, made[0].pk)
approve_all(made[1]); services.assign(ito, made[1].pk); services.complete(ito, made[1].pk)
approve_all(made[2]); services.assign(ito, made[2].pk); services.complete(ito, made[2].pk)
approve_all(made[3]); services.assign(takahashi, made[3].pk)
services.pull_back(made[4].requester, made[4].pk)
print("足した依頼:", [(r.pk, r.title) for r in made])

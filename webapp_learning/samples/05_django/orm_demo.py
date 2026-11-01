"""ORM の練習。使い方: .venv\\Scripts\\python.exe manage.py shell -c "import orm_demo"
（練習用のデータを作り直すので、何度動かしても同じ結果になる）"""

from datetime import date

from django.contrib.auth import get_user_model
from django.db.models import Count

from requests_app.models import Approval, Request

User = get_user_model()

# 0. 練習用のデータを作り直す
Request.objects.all().delete()
User.objects.filter(username__in=["012345", "012346", "012347"]).delete()
yamada = User.objects.create_user("012345", password="practice-only-1", first_name="太郎", last_name="山田")
sato = User.objects.create_user("012346", password="practice-only-2", first_name="花子", last_name="佐藤")
suzuki = User.objects.create_user("012347", password="practice-only-3", first_name="一郎", last_name="鈴木")

# 1. 作る（INSERT）
r1 = Request.objects.create(title="治具の製作", due_date=date(2026, 11, 10), requester=yamada)
r2 = Request.objects.create(title="部品の追加工", due_date=date(2026, 10, 31), requester=yamada, status=Request.Status.APPROVED)
r3 = Request.objects.create(title="棚の取り付け", due_date=date(2026, 11, 20), requester=sato, status=Request.Status.RETURNED)
Approval.objects.create(request=r1, order=1, approver=sato)
Approval.objects.create(request=r1, order=2, approver=suzuki)
print("作った件数:", Request.objects.count())

# 2. 絞る（WHERE）・並べる（ORDER BY）
circulating = Request.objects.filter(status=Request.Status.CIRCULATING)
print("回覧中:", [r.title for r in circulating])
print("山田さんの依頼（納期の近い順）:", [r.title for r in Request.objects.filter(requester=yamada).order_by("due_date")])

# 3. 裏で作られる SQL を見る
print("SQL:", circulating.query)

# 4. 1 件を取り出して、直す（UPDATE）
r = Request.objects.get(pk=r1.pk)
r.status = Request.Status.RETURNED
r.save()
print(r.title, "→", r.get_status_display())

# 5. 状態ごとの件数（GROUP BY）
counts = Request.objects.values("status").annotate(n=Count("id")).order_by("status")
print("状態ごとの件数:", {row["status"]: row["n"] for row in counts})

# 6. つながったデータ（回覧の順に、承認者の名前）
for a in Approval.objects.filter(request=r1).select_related("approver"):
    print(f"  {a.order} 番目: {a.approver.get_full_name()}（{a.get_decision_display()}）")

# 7. 無い物を get すると、エラーになる
try:
    Request.objects.get(pk=999)
except Request.DoesNotExist:
    print("999 番の依頼は無い")

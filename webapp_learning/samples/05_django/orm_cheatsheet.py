"""付録 B-16: Django ORM でよく使う書き方の早見表。
使い方: .venv\\Scripts\\python.exe manage.py shell -c "import orm_cheatsheet"
（練習用のデータを作り直すので、何度動かしても同じ結果になる）"""

from datetime import date

from django.contrib.auth import get_user_model
from django.db import transaction
from django.db.models import Count, F, Max, Min, Q

from requests_app.models import Approval, Request

User = get_user_model()
S = Request.Status

# 0. 練習用のデータを作り直す
Request.objects.all().delete()
User.objects.filter(username__in=["012345", "012346", "012347"]).delete()
yamada = User.objects.create_user("012345", password="practice-only-1", first_name="太郎", last_name="山田")
sato = User.objects.create_user("012346", password="practice-only-2", first_name="花子", last_name="佐藤")
suzuki = User.objects.create_user("012347", password="practice-only-3", first_name="一郎", last_name="鈴木")

# bulk_create: まとめて作る（1 回の SQL で入る）
Request.objects.bulk_create([
    Request(title="治具の製作", due_date=date(2026, 11, 10), requester=yamada),
    Request(title="部品の追加工", due_date=date(2026, 10, 31), requester=yamada, status=S.APPROVED),
    Request(title="棚の取り付け", due_date=date(2026, 11, 20), requester=sato, status=S.RETURNED),
    Request(title="工具の修理", due_date=date(2026, 12, 5), requester=suzuki),
    Request(title="治具の改造", due_date=date(2026, 11, 15), requester=sato),
])
jig = Request.objects.get(title="治具の製作")
Approval.objects.create(request=jig, order=1, approver=sato)
Approval.objects.create(request=jig, order=2, approver=suzuki)


def titles(qs):
    return [r.title for r in qs]


# 1. 数える・あるか・最初と最後
print("1. count:", Request.objects.count(), "/ exists:", Request.objects.filter(status=S.DONE).exists())
print("   first（納期の近い順）:", Request.objects.order_by("due_date").first().title)

# 2. 絞り込みの書き方（項目名__条件）
R = Request.objects  # 長いので、短い名前を付けておく
print("2. 件名に「治具」を含む:", titles(R.filter(title__icontains="治具").order_by("id")))
print("   状態がどれか:", titles(R.filter(status__in=[S.APPROVED, S.RETURNED]).order_by("id")))
nov = R.filter(due_date__year=2026, due_date__month=11).order_by("due_date")
print("   11 月が納期:", titles(nov))
print("   納期が 11/15 以降:", titles(R.filter(due_date__gte=date(2026, 11, 15)).order_by("due_date")))
print("   回覧中でない:", titles(R.exclude(status=S.CIRCULATING).order_by("id")))

# 3. Q で「または」・「ではない」を組み立てる
q = Q(requester=sato) | Q(due_date__lt=date(2026, 11, 1))
print("3. 佐藤さんの依頼、または 11 月より前:", titles(R.filter(q).order_by("id")))
not_yamada = R.filter(~Q(requester=yamada), status=S.CIRCULATING).order_by("id")
print("   山田さん以外で回覧中:", titles(not_yamada))

# 4. つながった先の項目で絞る（__ でたどる）
print("4. 鈴木さんが承認者に入っている:", titles(R.filter(approvals__approver=suzuki)))
print("   依頼者の名字が「佐藤」:", titles(R.filter(requester__last_name="佐藤").order_by("id")))

# 5. 並べる・ページに分ける（[ ] で LIMIT と OFFSET）
ordered = R.order_by("-due_date", "id")
print("5. 納期の遠い順の 1〜2 件目:", titles(ordered[:2]), "/ 3〜4 件目:", titles(ordered[2:4]))

# 6. 値だけを取り出す: values（辞書）・values_list（タプル。flat=True で 1 つの値）
print("6. values:", list(R.filter(requester=yamada).order_by("id").values("title", "status")))
print("   values_list:", list(R.order_by("id").values_list("title", flat=True)))
# distinct の前の order_by() は、Meta の並び順（作った日時）を外すため。外さないと重なりが消えない
names = R.order_by().values_list("requester__last_name", flat=True).distinct()
print("   distinct（重ならない依頼者）:", sorted(names))

# 7. 集計: aggregate（全体で 1 つ）・annotate（1 行ごと）
print("7. 納期の最小・最大:", R.aggregate(first=Min("due_date"), last=Max("due_date")))
users = User.objects.filter(username__in=["012345", "012346", "012347"])
per_user = users.annotate(n=Count("requests")).order_by("username")
print("   依頼者ごとの件数:", [(u.last_name, u.n) for u in per_user])
with_approvers = R.annotate(n=Count("approvals")).filter(n__gt=0)
print("   承認者の人数:", [(r.title, r.n) for r in with_approvers])

# 8. まとめて変える・消す（数が返る）
late = R.filter(status=S.CIRCULATING, due_date__gte=date(2026, 12, 1))
changed = late.update(status=S.IN_PROGRESS)
print("8. update で変えた件数:", changed, "/", R.get(title="工具の修理").get_status_display())

# 9. F: ほかの項目の値を使う（ここでは、承認の順番を 1 つずつ後ろへずらす）
Approval.objects.filter(request=jig).update(order=F("order") + 10)
orders = Approval.objects.filter(request=jig).order_by("order").values_list("order", flat=True)
print("9. F で順番をずらした:", list(orders))

# 10. get_or_create: あれば取り出し、無ければ作る（created で、どちらだったか分かる）
obj, created = R.get_or_create(
    title="治具の製作", defaults={"due_date": date(2026, 1, 1), "requester": yamada}
)
print("10. get_or_create:", obj.title, "作った？", created)

# 11. transaction.atomic: 途中で失敗したら、全部なかったことにする
try:
    with transaction.atomic():
        Request.objects.filter(title="治具の改造").update(status=S.DONE)
        raise RuntimeError("わざと失敗")
except RuntimeError:
    pass
print("11. 失敗したので元のまま:", Request.objects.get(title="治具の改造").get_status_display())

# 12. 消す（delete は、消した件数と、種類ごとの件数を返す。回覧も一緒に消える）
print("12. delete:", Request.objects.filter(title="治具の製作").delete())

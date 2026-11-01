"""練習用の社員を作る（開発用のパソコンだけで使う）。使い方: .\\dev.ps1 seed_demo

本番では使わない。本番の社員は import_employees で取り込み、パスワードは本人が設定する。
"""

from django.conf import settings
from django.core.management.base import BaseCommand, CommandError

from accounts.models import User

PEOPLE = [
    # 従業員番号, 氏名, 所属, 役職, 権限
    ("012345", "山田 太郎", "設計部", "主任", User.Role.MEMBER),
    ("012346", "佐藤 花子", "設計部", "課長", User.Role.MEMBER),
    ("012347", "鈴木 一郎", "設計部", "部長", User.Role.MEMBER),
    ("020001", "高橋 三郎", "設備課", "係長", User.Role.WORKER),
    ("090001", "管理 次郎", "情報システム部", "", User.Role.ADMIN),
]
PASSWORD = "practice-pass-2026"  # 練習用。本物のパスワードには使わない


class Command(BaseCommand):
    help = "練習用の社員を作る（開発用のパソコンだけ）"

    def handle(self, **options):
        if not settings.DEBUG:
            raise CommandError("DEBUG が true のとき（開発用のパソコン）だけ使えます。")
        for no, name, dept, title, role in PEOPLE:
            user, _ = User.objects.update_or_create(
                employee_no=no,
                defaults={"name": name, "dept": dept, "job_title": title, "role": role,
                          "email": f"{no}@example.com", "is_staff": role == User.Role.ADMIN,
                          "is_superuser": role == User.Role.ADMIN},
            )
            user.set_password(PASSWORD)
            user.save()
        self.stdout.write(f"{len(PEOPLE)} 人を作りました。パスワードは全員 {PASSWORD} です（練習用）。")

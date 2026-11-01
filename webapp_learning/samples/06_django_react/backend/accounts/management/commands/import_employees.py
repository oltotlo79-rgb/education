"""社員の CSV を取り込む。使い方: python manage.py import_employees 社員.csv [--dry-run]

・1 行でも間違いがあれば、1 件も取り込まない（全部か、何もしないか）
・同じ従業員番号の社員がいれば、氏名などを書きかえる。いなければ作る（パスワードは未設定）
・CSV は Excel の「CSV UTF-8（コンマ区切り）」で保存した物（04 の 5 章）
"""

import csv
import re
from pathlib import Path

from django.core.management.base import BaseCommand, CommandError
from django.core.validators import validate_email
from django.core.exceptions import ValidationError
from django.db import transaction

from accounts.models import User

COLUMNS = ["従業員番号", "氏名", "所属", "役職", "メールアドレス"]


class Command(BaseCommand):
    help = "社員の CSV を取り込む"

    def add_arguments(self, parser):
        parser.add_argument("path")
        parser.add_argument("--dry-run", action="store_true", help="確かめるだけで、取り込まない")

    def handle(self, path, dry_run, **options):
        rows, errors = self.read(Path(path))
        if errors:
            for e in errors:
                self.stderr.write(f"× {e}")
            raise CommandError(f"{len(errors)} 件の間違いがあるので、取り込みませんでした。")
        if dry_run:
            self.stdout.write(f"間違いはありません（{len(rows)} 件）。--dry-run なので、取り込んでいません。")
            return
        created = updated = 0
        with transaction.atomic():
            for no, name, dept, title, email in rows:
                _, is_new = User.objects.update_or_create(
                    employee_no=no, defaults={"name": name, "dept": dept, "job_title": title, "email": email}
                )
                created += is_new
                updated += not is_new
        self.stdout.write(self.style.SUCCESS(f"取り込みました（新しく {created} 件・書きかえ {updated} 件）。"))

    def read(self, path: Path):
        rows, errors, seen = [], [], set()
        try:
            with path.open(encoding="utf-8-sig", newline="") as f:
                reader = csv.DictReader(f)
                if reader.fieldnames != COLUMNS:
                    return [], [f"1 行目の見出しは {','.join(COLUMNS)} にしてください（今: {reader.fieldnames}）"]
                for n, row in enumerate(reader, start=2):
                    no, name, dept, title, email = (row[c].strip() for c in COLUMNS)
                    if not re.fullmatch(r"\d{6}", no):
                        errors.append(f"{n} 行目: 従業員番号は 6 桁の数字にしてください（{no!r}）")
                    elif no in seen:
                        errors.append(f"{n} 行目: 従業員番号 {no} が 2 回出てきます")
                    elif not name:
                        errors.append(f"{n} 行目: 氏名が空です")
                    else:
                        try:
                            validate_email(email)
                        except ValidationError:
                            errors.append(f"{n} 行目: メールアドレスの形が違います（{email!r}）")
                            continue
                        seen.add(no)
                        rows.append((no, name, dept, title, email))
        except UnicodeDecodeError:
            return [], ["文字コードが UTF-8 ではありません。Excel で「CSV UTF-8」として保存し直してください。"]
        return rows, errors

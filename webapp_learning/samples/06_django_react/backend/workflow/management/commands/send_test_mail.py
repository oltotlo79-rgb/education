"""テストのメールを 1 通送る（メールの道を確かめる。09 の巻）。使い方: .\\dev.ps1 send_test_mail --to 自分のアドレス

サーバーでは onprem の 05 の manage.ps1 から動かす。検証のサーバーでは、MAIL_REDIRECT_TO の宛先に届く。
アプリの不具合か、メールの道（SMTP リレー・ネットワーク）の問題かを分けるために使う。
"""

from django.conf import settings
from django.core.mail import send_mail
from django.core.management.base import BaseCommand, CommandError
from django.utils import timezone


class Command(BaseCommand):
    help = "テストのメールを 1 通送る"

    def add_arguments(self, parser):
        parser.add_argument("--to", required=True, help="送り先（自分のアドレス）")

    def handle(self, to, **options):
        if settings.EMAIL_BACKEND.endswith("smtp.EmailBackend"):
            where = f"{settings.EMAIL_HOST}:{settings.EMAIL_PORT}（STARTTLS: {'使う' if settings.EMAIL_USE_TLS else '使わない'}）"
        else:
            where = settings.EMAIL_BACKEND
        subject = "【依頼・承認】送信テスト"
        body = f"依頼・承認アプリからの、送信テストです。\n\n送った時刻: {timezone.localtime():%Y-%m-%d %H:%M}\n渡した先: {where}\n"
        if settings.MAIL_REDIRECT_TO:  # 検証のサーバーでは、services._mail と同じく宛先を差し替える
            body = f"（検証のメールです。本来の宛先: {to}）\n\n{body}"
            subject, to = f"【検証】{subject}", settings.MAIL_REDIRECT_TO
        try:
            send_mail(subject, body, None, [to])
        except Exception as e:  # 原因の種類（例外の名前）も出して、調べやすくする
            raise CommandError(f"送れませんでした（渡した先: {where}）: {type(e).__name__}: {e}")
        self.stdout.write(f"渡しました（宛先: {to}・渡した先: {where}）。届いたかを、受け取る側で確かめてください。")

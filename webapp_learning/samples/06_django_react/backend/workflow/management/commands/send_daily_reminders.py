"""毎朝 8:00 の通知。サーバーでは、タスク スケジューラが manage.ps1 から動かす（onprem の 05 の 12.1）

1. 承認のお願いを送ってから 3 日たっても止まっている回覧 → 承認者に送り直す（前の URL は使えなくなる）
2. 希望の納期まで 3 日以内なのに、まだ終わっていない依頼 → 依頼者と担当者に知らせる
"""

from datetime import timedelta

from django.core.management.base import BaseCommand
from django.utils import timezone

from workflow import services
from workflow.models import Request, Step

STALE_DAYS = 3
DUE_SOON_DAYS = 3


class Command(BaseCommand):
    help = "毎朝の通知（止まっている承認・納期の近い依頼）"

    def handle(self, **options):
        now = timezone.now()
        stale = Step.objects.filter(
            decision=Step.Decision.WAITING,
            request__status=Request.Status.CIRCULATING,
            sent_at__lt=now - timedelta(days=STALE_DAYS),
        )
        resent = 0
        for step in stale:
            services.resend(None, step.pk)
            resent += 1

        today = timezone.localdate()
        due_soon = Request.objects.filter(
            status__in=[Request.Status.APPROVED, Request.Status.IN_PROGRESS],
            due_date__lte=today + timedelta(days=DUE_SOON_DAYS),
        ).select_related("requester", "assignee")
        for r in due_soon:
            services.notify_due_soon(r)
        self.stdout.write(f"送り直した承認のお願い: {resent} 件 / 納期が近い依頼: {len(due_soon)} 件")

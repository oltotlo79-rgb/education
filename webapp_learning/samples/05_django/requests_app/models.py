from django.conf import settings
from django.db import models


class Request(models.Model):
    """依頼 1 件"""

    class Status(models.TextChoices):
        # データベースに入る値（英字）と、画面に出す名前（日本語）
        CIRCULATING = "circulating", "回覧中"
        RETURNED = "returned", "差し戻し中"
        PULLED_BACK = "pulled_back", "引き戻し中"
        APPROVED = "approved", "回覧完了"
        IN_PROGRESS = "in_progress", "対応中"
        DONE = "done", "完了"
        WITHDRAWN = "withdrawn", "取り下げ"

    title = models.CharField("件名", max_length=100)
    detail = models.TextField("内容", blank=True)
    due_date = models.DateField("希望の納期")
    status = models.CharField("状態", max_length=20, choices=Status.choices, default=Status.CIRCULATING)
    requester = models.ForeignKey(
        settings.AUTH_USER_MODEL, verbose_name="依頼者", on_delete=models.PROTECT, related_name="requests"
    )
    created_at = models.DateTimeField("作った日時", auto_now_add=True)

    class Meta:
        verbose_name = "依頼"
        verbose_name_plural = "依頼"
        ordering = ["-created_at"]  # 新しい順

    def __str__(self) -> str:
        return f"{self.pk} {self.title}"


class Approval(models.Model):
    """回覧の 1 人分（だれが、何番目に、どうしたか）"""

    class Decision(models.TextChoices):
        WAITING = "waiting", "待ち"
        APPROVED = "approved", "承認"
        RETURNED = "returned", "差し戻し"

    request = models.ForeignKey(Request, verbose_name="依頼", on_delete=models.CASCADE, related_name="approvals")
    order = models.PositiveSmallIntegerField("順番")
    approver = models.ForeignKey(settings.AUTH_USER_MODEL, verbose_name="承認者", on_delete=models.PROTECT)
    decision = models.CharField("結果", max_length=20, choices=Decision.choices, default=Decision.WAITING)
    decided_at = models.DateTimeField("決めた日時", null=True, blank=True)

    class Meta:
        verbose_name = "回覧"
        verbose_name_plural = "回覧"
        ordering = ["request", "order"]
        constraints = [
            # 1 つの依頼の中で、順番は重ならない
            models.UniqueConstraint(fields=["request", "order"], name="unique_order_per_request"),
        ]

    def __str__(self) -> str:
        return f"{self.request_id} の {self.order} 番目"

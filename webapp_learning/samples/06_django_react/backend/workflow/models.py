from django.conf import settings
from django.db import models


class Request(models.Model):
    """依頼 1 件"""

    class Status(models.TextChoices):
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
    requester = models.ForeignKey(settings.AUTH_USER_MODEL, verbose_name="依頼者", on_delete=models.PROTECT, related_name="requests")
    assignee = models.ForeignKey(
        settings.AUTH_USER_MODEL, verbose_name="担当者", on_delete=models.PROTECT, null=True, blank=True, related_name="assigned"
    )
    drawing = models.JSONField("図面", null=True, blank=True)
    return_comments = models.TextField("差し戻しのコメント", blank=True)  # 差し戻すたびに追記する
    round = models.PositiveSmallIntegerField("回覧の回数", default=1)      # 出し直すたびに 1 増える
    created_at = models.DateTimeField("作った日時", auto_now_add=True)
    updated_at = models.DateTimeField("直した日時", auto_now=True)

    class Meta:
        verbose_name = "依頼"
        verbose_name_plural = "依頼"
        ordering = ["-created_at"]

    def __str__(self) -> str:
        return f"{self.pk} {self.title}"


class Step(models.Model):
    """回覧の 1 人分。承認の URL の指紋も、ここに持つ"""

    class Decision(models.TextChoices):
        WAITING = "waiting", "待ち"
        APPROVED = "approved", "承認"
        RETURNED = "returned", "差し戻し"
        CANCELLED = "cancelled", "取り消し"  # 引き戻し・差し戻しで、回ってこなくなった

    request = models.ForeignKey(Request, verbose_name="依頼", on_delete=models.CASCADE, related_name="steps")
    round = models.PositiveSmallIntegerField("回覧の回数")
    order = models.PositiveSmallIntegerField("順番")
    approver = models.ForeignKey(settings.AUTH_USER_MODEL, verbose_name="承認者", on_delete=models.PROTECT)
    decision = models.CharField("結果", max_length=10, choices=Decision.choices, default=Decision.WAITING)
    decided_at = models.DateTimeField("決めた日時", null=True, blank=True)
    token_hash = models.CharField("URL の指紋", max_length=64, blank=True, db_index=True)
    token_expires_at = models.DateTimeField("URL の期限", null=True, blank=True)
    sent_at = models.DateTimeField("メールを送った日時", null=True, blank=True)

    class Meta:
        verbose_name = "回覧"
        verbose_name_plural = "回覧"
        ordering = ["request", "round", "order"]
        constraints = [models.UniqueConstraint(fields=["request", "round", "order"], name="unique_step")]

    def __str__(self) -> str:
        return f"{self.request_id} の {self.round} 回目 {self.order} 番目"


class Attachment(models.Model):
    """添付ファイル（JPG・PNG・PDF。1 ファイル 10MB まで）"""

    request = models.ForeignKey(Request, verbose_name="依頼", on_delete=models.CASCADE, related_name="attachments")
    file = models.FileField("ファイル", upload_to="attachments/%Y/%m/")
    original_name = models.CharField("元の名前", max_length=200)
    size = models.PositiveIntegerField("大きさ（バイト）")
    uploaded_at = models.DateTimeField("付けた日時", auto_now_add=True)

    class Meta:
        verbose_name = "添付ファイル"
        verbose_name_plural = "添付ファイル"


class History(models.Model):
    """だれが、いつ、何をしたかの記録（あとから変えない）"""

    request = models.ForeignKey(Request, verbose_name="依頼", on_delete=models.CASCADE, related_name="histories")
    at = models.DateTimeField("日時", auto_now_add=True)
    actor = models.ForeignKey(settings.AUTH_USER_MODEL, verbose_name="した人", on_delete=models.PROTECT, null=True)
    action = models.CharField("したこと", max_length=50)
    note = models.TextField("メモ", blank=True)

    class Meta:
        verbose_name = "記録"
        verbose_name_plural = "記録"
        ordering = ["at", "id"]

    def __str__(self) -> str:
        return f"{self.action}"

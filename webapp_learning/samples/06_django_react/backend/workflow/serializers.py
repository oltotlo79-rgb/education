import unicodedata

from django.core.exceptions import ValidationError as DjangoValidationError
from rest_framework import serializers

from .models import Attachment, History, Request, Step
from .validators import validate_drawing


class StepSerializer(serializers.ModelSerializer):
    approver_id = serializers.IntegerField(source="approver.pk")
    approver_name = serializers.CharField(source="approver.name")
    approver_dept = serializers.CharField(source="approver.dept")
    decision_label = serializers.CharField(source="get_decision_display")

    class Meta:
        model = Step
        fields = ["round", "order", "approver_id", "approver_name", "approver_dept", "decision", "decision_label", "decided_at"]


class AttachmentSerializer(serializers.ModelSerializer):
    class Meta:
        model = Attachment
        fields = ["id", "original_name", "size", "uploaded_at"]


class HistorySerializer(serializers.ModelSerializer):
    actor_name = serializers.CharField(source="actor.name", default="")

    class Meta:
        model = History
        fields = ["at", "actor_name", "action", "note"]


class RequestListSerializer(serializers.ModelSerializer):
    status_label = serializers.CharField(source="get_status_display")
    waiting_for = serializers.SerializerMethodField()

    class Meta:
        model = Request
        fields = ["id", "title", "due_date", "status", "status_label", "waiting_for", "created_at"]

    def get_waiting_for(self, r: Request) -> str:
        """回覧中なら、今だれの承認を待っているか"""
        if r.status != Request.Status.CIRCULATING:
            return ""
        step = next((s for s in r.steps.all() if s.round == r.round and s.decision == Step.Decision.WAITING), None)
        return f"回覧者{step.order} {step.approver.name}" if step else ""


class RequestDetailSerializer(RequestListSerializer):
    requester_name = serializers.CharField(source="requester.name")
    assignee_name = serializers.CharField(source="assignee.name", default="")
    steps = serializers.SerializerMethodField()
    attachments = AttachmentSerializer(many=True)
    histories = HistorySerializer(many=True)

    class Meta(RequestListSerializer.Meta):
        fields = RequestListSerializer.Meta.fields + [
            "detail", "requester_name", "assignee_name", "drawing", "return_comments", "round", "steps",
            "attachments", "histories",
        ]

    def get_steps(self, r: Request):
        return StepSerializer([s for s in r.steps.all() if s.round == r.round], many=True).data


class RequestInputSerializer(serializers.Serializer):
    """依頼を出す・出し直すときに受け取る物"""

    title = serializers.CharField(max_length=100)
    detail = serializers.CharField(allow_blank=True, required=False, default="")
    due_date = serializers.DateField()
    approver_ids = serializers.ListField(child=serializers.IntegerField(), min_length=1, max_length=3)
    drawing = serializers.JSONField(required=False, allow_null=True, default=None)

    def validate_title(self, value: str) -> str:
        value = value.strip()
        if not value:
            raise serializers.ValidationError("件名を入れてください。")
        # 改行などの制御文字は断る。メールの件名に入ると、保存できてもメールを送れなくなる（10 の巻）
        if any(unicodedata.category(c) == "Cc" for c in value):
            raise serializers.ValidationError("件名に、改行などは入れられません。")
        return value

    def validate_drawing(self, value):
        try:
            validate_drawing(value)
        except DjangoValidationError as e:
            raise serializers.ValidationError(e.messages)
        return value

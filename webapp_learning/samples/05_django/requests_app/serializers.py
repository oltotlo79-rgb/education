from rest_framework import serializers

from .models import Request


class RequestSerializer(serializers.ModelSerializer):
    """依頼 ⇔ JSON の変換と、入力の確かめ"""

    status_label = serializers.CharField(source="get_status_display", read_only=True)
    requester_name = serializers.CharField(source="requester.get_full_name", read_only=True)

    class Meta:
        model = Request
        fields = ["id", "title", "detail", "due_date", "status", "status_label", "requester_name", "created_at"]
        read_only_fields = ["status", "created_at"]  # 状態は、画面から直接は変えさせない

    def validate_title(self, value: str) -> str:
        if not value.strip():
            raise serializers.ValidationError("件名を入れてください")
        return value

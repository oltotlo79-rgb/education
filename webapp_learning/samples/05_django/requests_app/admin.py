from django.contrib import admin

from .models import Approval, Request


class ApprovalInline(admin.TabularInline):
    """依頼の画面の中に、回覧の表を並べる"""

    model = Approval
    extra = 0


@admin.register(Request)
class RequestAdmin(admin.ModelAdmin):
    list_display = ["id", "title", "status", "due_date", "requester", "created_at"]  # 一覧に出す列
    list_filter = ["status"]                                                          # 右の絞り込み
    search_fields = ["title", "detail"]                                               # 検索の対象
    inlines = [ApprovalInline]

from django.contrib import admin

from .models import Attachment, History, Request, Step


class StepInline(admin.TabularInline):
    model = Step
    extra = 0
    fields = ["round", "order", "approver", "decision", "decided_at", "sent_at", "token_expires_at"]
    readonly_fields = fields  # 管理画面からは変えない（変えるのは services の操作だけ）
    can_delete = False

    def has_add_permission(self, request, obj=None):
        return False


class HistoryInline(admin.TabularInline):
    model = History
    extra = 0
    readonly_fields = ["at", "actor", "action", "note"]
    can_delete = False

    def has_add_permission(self, request, obj=None):
        return False


@admin.register(Request)
class RequestAdmin(admin.ModelAdmin):
    list_display = ["id", "title", "status", "due_date", "requester", "assignee", "created_at"]
    list_filter = ["status"]
    search_fields = ["title", "detail"]
    readonly_fields = ["status", "round", "return_comments", "created_at", "updated_at"]
    inlines = [StepInline, HistoryInline]


admin.site.register(Attachment)

"""React の画面が呼ぶ API（JSON）。状態を変える処理は services に任せる"""

from django.contrib.auth import get_user_model
from django.core.exceptions import ValidationError as DjangoValidationError
from django.db.models import Count, Q
from django.http import FileResponse
from django.shortcuts import get_object_or_404
from rest_framework import status
from rest_framework.decorators import api_view, parser_classes
from rest_framework.exceptions import PermissionDenied
from rest_framework.parsers import MultiPartParser
from rest_framework.response import Response

from . import services
from .models import Attachment, Request
from .serializers import (
    AttachmentSerializer,
    RequestDetailSerializer,
    RequestInputSerializer,
    RequestListSerializer,
)
from .validators import validate_upload

User = get_user_model()


def _visible_requests(user):
    """その人が見てよい依頼: 自分が出した・回覧に入っている・担当者と管理者は全部"""
    qs = Request.objects.select_related("requester", "assignee").prefetch_related("steps__approver")
    if user.role in (User.Role.WORKER, User.Role.ADMIN):
        return qs
    return qs.filter(Q(requester=user) | Q(steps__approver=user)).distinct()


def _error(e: Exception) -> Response:
    return Response({"detail": str(e)}, status=status.HTTP_400_BAD_REQUEST)


@api_view(["GET", "POST"])
def requests_view(request):
    if request.method == "GET":
        qs = _visible_requests(request.user)
        if request.query_params.get("mine") == "1":
            qs = qs.filter(requester=request.user)
        return Response(RequestListSerializer(qs, many=True).data)
    data = RequestInputSerializer(data=request.data)
    data.is_valid(raise_exception=True)
    try:
        r = services.submit(request.user, **data.validated_data)
    except services.WorkflowError as e:
        return _error(e)
    return Response({"id": r.pk}, status=status.HTTP_201_CREATED)


@api_view(["GET"])
def request_detail(request, pk: int):
    r = get_object_or_404(_visible_requests(request.user).prefetch_related("attachments", "histories__actor"), pk=pk)
    return Response(RequestDetailSerializer(r).data)


@api_view(["POST"])
def request_action(request, pk: int, action: str):
    """引き戻し・出し直し・取り下げ・担当・完了"""
    try:
        if action == "pull-back":
            services.pull_back(request.user, pk)
        elif action == "withdraw":
            services.withdraw(request.user, pk)
        elif action == "resubmit":
            data = RequestInputSerializer(data=request.data)
            data.is_valid(raise_exception=True)
            services.resubmit(request.user, pk, **data.validated_data)
        elif action == "assign":
            services.assign(request.user, pk)
        elif action == "complete":
            services.complete(request.user, pk)
        else:
            return Response({"detail": "知らない操作です。"}, status=status.HTTP_404_NOT_FOUND)
    except Request.DoesNotExist:
        return Response({"detail": "依頼がありません。"}, status=status.HTTP_404_NOT_FOUND)
    except services.WorkflowError as e:
        return _error(e)
    return Response({"ok": True})


@api_view(["POST"])
@parser_classes([MultiPartParser])
def upload_attachment(request, pk: int):
    r = get_object_or_404(Request, pk=pk, requester=request.user)
    if r.status not in (Request.Status.CIRCULATING, Request.Status.RETURNED, Request.Status.PULLED_BACK):
        return Response({"detail": "この状態の依頼には、添付できません。"}, status=status.HTTP_400_BAD_REQUEST)
    f = request.FILES.get("file")
    if f is None:
        return Response({"detail": "ファイルがありません。"}, status=status.HTTP_400_BAD_REQUEST)
    head = f.read(8)
    f.seek(0)
    try:
        validate_upload(f.name, head, f.size)
    except DjangoValidationError as e:
        return Response({"detail": e.messages[0]}, status=status.HTTP_400_BAD_REQUEST)
    a = Attachment.objects.create(request=r, file=f, original_name=f.name, size=f.size)
    return Response(AttachmentSerializer(a).data, status=status.HTTP_201_CREATED)


@api_view(["GET"])
def download_attachment(request, attachment_id: int):
    a = get_object_or_404(Attachment.objects.select_related("request"), pk=attachment_id)
    if not _visible_requests(request.user).filter(pk=a.request_id).exists():
        raise PermissionDenied("この添付ファイルは見られません。")
    return FileResponse(a.file.open("rb"), as_attachment=True, filename=a.original_name)


@api_view(["GET"])
def employees(request):
    """回覧者を選ぶための社員の検索（氏名・所属・役職の一部）"""
    q = request.query_params.get("q", "").strip()
    qs = User.objects.filter(is_active=True).exclude(pk=request.user.pk)
    if q:
        qs = qs.filter(Q(name__icontains=q) | Q(dept__icontains=q) | Q(job_title__icontains=q))
    return Response([{"id": u.pk, "name": u.name, "dept": u.dept, "job_title": u.job_title} for u in qs[:20]])


@api_view(["GET"])
def stats(request):
    """グラフの元: 状態ごとの件数と、担当者ごとの完了の件数"""
    by_status = Request.objects.values("status").annotate(n=Count("id"))
    labels = dict(Request.Status.choices)
    done_by = (
        Request.objects.filter(status=Request.Status.DONE, assignee__isnull=False)
        .values("assignee__name").annotate(n=Count("id")).order_by("-n")
    )
    return Response({
        "by_status": [{"status": labels[row["status"]], "count": row["n"]} for row in by_status],
        "done_by_assignee": [{"name": row["assignee__name"], "count": row["n"]} for row in done_by],
    })


@api_view(["GET"])
def me(request):
    u = request.user
    return Response({"id": u.pk, "employee_no": u.employee_no, "name": u.name, "role": u.role})

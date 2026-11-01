from django.contrib.auth.decorators import login_required
from django.shortcuts import render
from rest_framework import permissions, viewsets

from .models import Request
from .serializers import RequestSerializer


@login_required
def request_list(request):
    """自分の依頼の一覧（Django のテンプレートで HTML を作る）"""
    items = Request.objects.filter(requester=request.user)
    return render(request, "requests_app/request_list.html", {"items": items})


class RequestViewSet(viewsets.ModelViewSet):
    """依頼の API（JSON）。一覧・1 件・作る・直す・消すを、まとめて用意する"""

    serializer_class = RequestSerializer
    permission_classes = [permissions.IsAuthenticated]

    def get_queryset(self):
        # 自分の依頼だけ（ほかの人の依頼は、見えも直せもしない）
        return Request.objects.filter(requester=self.request.user).select_related("requester")

    def perform_create(self, serializer):
        # 依頼者は、送られてきた値ではなく、ログインしている人にする
        serializer.save(requester=self.request.user)

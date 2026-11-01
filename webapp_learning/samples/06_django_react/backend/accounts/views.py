"""React の画面のための、ログイン・ログアウト（Django のセッションを使う）"""

import json

from django.contrib.auth import authenticate, login, logout
from django.http import JsonResponse
from django.views.decorators.http import require_POST


@require_POST
def login_view(request):
    """CSRF の確かめは、Django の CsrfViewMiddleware が行う（onprem の 05 の 8.2）"""
    data = json.loads(request.body or b"{}")
    user = authenticate(request, employee_no=data.get("employee_no", ""), password=data.get("password", ""))
    if user is None:
        return JsonResponse({"detail": "従業員番号かパスワードが違います。"}, status=400)
    login(request, user)
    return JsonResponse({"name": user.name})


@require_POST
def logout_view(request):
    logout(request)
    return JsonResponse({"ok": True})

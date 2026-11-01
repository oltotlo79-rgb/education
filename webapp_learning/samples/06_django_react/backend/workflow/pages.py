"""React の画面（index.html）を返す所と、動いているかの確かめ（/health）"""

from django.http import HttpResponse, JsonResponse
from django.utils.decorators import method_decorator
from django.views.decorators.csrf import ensure_csrf_cookie
from django.views.generic import TemplateView


@method_decorator(ensure_csrf_cookie, name="dispatch")
class IndexView(TemplateView):
    """React の成果物の index.html を返す。CSRF の札（クッキー）もここで渡す"""

    template_name = "index.html"


@ensure_csrf_cookie
def csrf(request):
    """開発中（Vite の画面から開いたとき）に、CSRF の札を受け取るための入口"""
    return HttpResponse(status=204)


def health(request):
    """動いているかの確かめ（onprem の 06 の check.ps1 が使う）"""
    return JsonResponse({"status": "ok"})

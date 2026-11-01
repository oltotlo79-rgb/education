"""メールの URL から開く、承認の画面。ログインは要らない（URL の文字列が、本人の印になる）"""

from django.shortcuts import render
from django.views.decorators.http import require_http_methods

from . import services


@require_http_methods(["GET", "POST"])
def approve_page(request, token: str):
    if request.method == "POST":
        try:
            if request.POST.get("decision") == "approve":
                r = services.approve(token)
                message = f"「{r.title}」を承認しました。"
            else:
                r = services.return_request(token, request.POST.get("comment", ""))
                message = f"「{r.title}」を差し戻しました。"
        except services.WorkflowError as e:
            return render(request, "workflow/approve_result.html", {"message": str(e), "ok": False}, status=400)
        return render(request, "workflow/approve_result.html", {"message": message, "ok": True})

    check = services.check_token(token)
    if check.step is None:
        return render(request, "workflow/approve_result.html", {"message": check.message, "ok": False}, status=410)
    r = check.step.request
    return render(request, "workflow/approve.html", {"step": check.step, "r": r, "attachments": r.attachments.all()})

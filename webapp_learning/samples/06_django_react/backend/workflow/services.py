"""依頼の状態を変える処理を、ここに集める。

画面（API・承認の画面）からは、ここの関数だけを呼ぶ。状態を直接書きかえない。
どの関数も、1 つのトランザクションの中で動き、途中で失敗したら全部取り消す。
メールは、データベースへの保存が確定してから送る（transaction.on_commit）。
"""

import hashlib
import secrets
from dataclasses import dataclass
from datetime import timedelta

from django.conf import settings
from django.contrib.auth import get_user_model
from django.core.mail import send_mail
from django.db import transaction
from django.template.loader import render_to_string
from django.utils import timezone

from .models import History, Request, Step

User = get_user_model()
S = Request.Status
D = Step.Decision
TOKEN_DAYS = 14
MAX_APPROVERS = 3


class WorkflowError(Exception):
    """できない操作。文は、そのまま画面に出してよい"""


# ---------- 承認の URL ----------

def _hash(token: str) -> str:
    return hashlib.sha256(token.encode()).hexdigest()


def _issue_url(step: Step) -> str:
    """推測できない文字列を作り、指紋だけを保存して、URL を返す（前の URL は使えなくなる）"""
    token = secrets.token_urlsafe(32)
    step.token_hash = _hash(token)
    step.token_expires_at = timezone.now() + timedelta(days=TOKEN_DAYS)
    step.sent_at = timezone.now()
    step.save(update_fields=["token_hash", "token_expires_at", "sent_at"])
    return f"{settings.APP_BASE_URL}/approve/{token}/"


@dataclass
class TokenCheck:
    step: Step | None
    message: str = ""  # 使えないときの理由


def check_token(token: str, *, lock: bool = False) -> TokenCheck:
    """URL の文字列を調べる。使えなければ、理由の文を返す"""
    qs = Step.objects.select_related("request", "approver", "request__requester")
    if lock:
        qs = qs.select_for_update()
    step = qs.filter(token_hash=_hash(token)).first()
    if step is None:
        return TokenCheck(None, "この URL は使えません。")
    if step.decision == D.APPROVED:
        return TokenCheck(None, "この依頼は、承認済みです。")
    if step.decision == D.RETURNED:
        return TokenCheck(None, "この依頼は、差し戻し済みです。")
    if step.decision == D.CANCELLED:
        if step.request.status == S.PULLED_BACK:
            return TokenCheck(None, "依頼者により引き戻されました。")
        return TokenCheck(None, "この URL は、取り消されました。")
    if step.token_expires_at and step.token_expires_at < timezone.now():
        return TokenCheck(None, "この URL は、期限が切れています。管理者に再送を頼んでください。")
    return TokenCheck(step)


# ---------- メール ----------

def _mail(to: str, subject: str, template: str, context: dict) -> None:
    body = render_to_string(f"workflow/mail/{template}", context)
    subject = f"【依頼・承認】{subject}"
    if settings.MAIL_REDIRECT_TO:  # 検証: 本物の宛先には送らず、決まった宛先へ
        body = f"（検証のメールです。本来の宛先: {to}）\n\n{body}"
        subject, to = f"【検証】{subject}", settings.MAIL_REDIRECT_TO
    # 保存が確定してから送る。途中で取り消されたら、送らない
    transaction.on_commit(lambda: send_mail(subject, body, None, [to]))


def _send_approval_mail(step: Step) -> None:
    url = _issue_url(step)
    r = step.request
    _mail(step.approver.email, f"承認のお願い: {r.title}", "approval_request.txt",
          {"step": step, "r": r, "url": url, "days": TOKEN_DAYS})


# ---------- 状態を変える操作 ----------

def _log(r: Request, actor, action: str, note: str = "") -> None:
    History.objects.create(request=r, actor=actor, action=action, note=note)


def _check_approvers(requester, approver_ids: list[int]) -> list:
    if not 1 <= len(approver_ids) <= MAX_APPROVERS:
        raise WorkflowError(f"回覧者は 1〜{MAX_APPROVERS} 人にしてください。")
    if len(set(approver_ids)) != len(approver_ids):
        raise WorkflowError("同じ人を 2 回は選べません。")
    if requester.pk in approver_ids:
        raise WorkflowError("自分自身は回覧者にできません。")
    users = {u.pk: u for u in User.objects.filter(pk__in=approver_ids, is_active=True)}
    if len(users) != len(approver_ids):
        raise WorkflowError("回覧者に、いない社員が入っています。")
    return [users[i] for i in approver_ids]


def _start_round(r: Request, approvers: list) -> None:
    steps = [Step.objects.create(request=r, round=r.round, order=i, approver=a) for i, a in enumerate(approvers, start=1)]
    _send_approval_mail(steps[0])


@transaction.atomic
def submit(requester, *, title: str, detail: str, due_date, approver_ids: list[int], drawing=None) -> Request:
    """依頼を出す → 回覧中。1 人目に承認のお願いのメール"""
    approvers = _check_approvers(requester, approver_ids)
    r = Request.objects.create(requester=requester, title=title, detail=detail, due_date=due_date, drawing=drawing)
    _start_round(r, approvers)
    _log(r, requester, "依頼を出した")
    return r


@transaction.atomic
def approve(token: str) -> Request:
    """承認 → 次の人へ。最後の人なら回覧完了"""
    check = check_token(token, lock=True)
    if check.step is None:
        raise WorkflowError(check.message)
    step = check.step
    r = Request.objects.select_for_update().get(pk=step.request_id)  # 同時に押されても 1 回だけ進める
    # 指紋は消さずに残す。同じ URL をもう一度開いた人に「承認済みです」と知らせるため
    step.decision, step.decided_at = D.APPROVED, timezone.now()
    step.save(update_fields=["decision", "decided_at"])
    _log(r, step.approver, f"承認（回覧者{step.order}）")

    nxt = r.steps.filter(round=r.round, decision=D.WAITING).order_by("order").first()
    if nxt:
        _send_approval_mail(nxt)
    else:
        r.status = S.APPROVED
        r.save(update_fields=["status", "updated_at"])
        _mail(r.requester.email, f"回覧完了: {r.title}", "approved.txt", {"r": r})
        for worker in User.objects.filter(role=User.Role.WORKER, is_active=True):
            _mail(worker.email, f"新しい依頼: {r.title}", "new_request.txt", {"r": r})
    return r


@transaction.atomic
def return_request(token: str, comment: str) -> Request:
    """差し戻し → 差し戻し中。コメントは【日時 回覧者n 氏名】を付けて追記する"""
    if not comment.strip():
        raise WorkflowError("差し戻すときは、理由を書いてください。")
    check = check_token(token, lock=True)
    if check.step is None:
        raise WorkflowError(check.message)
    step = check.step
    r = Request.objects.select_for_update().get(pk=step.request_id)
    now = timezone.localtime()
    step.decision, step.decided_at = D.RETURNED, now
    step.save(update_fields=["decision", "decided_at"])
    r.steps.filter(round=r.round, decision=D.WAITING).update(decision=D.CANCELLED)
    line = f"【{now:%m/%d %H:%M} 回覧者{step.order} {step.approver.name}】{comment.strip()}"
    r.return_comments = f"{r.return_comments}\n{line}".strip()
    r.status = S.RETURNED
    r.save(update_fields=["status", "return_comments", "updated_at"])
    _log(r, step.approver, f"差し戻し（回覧者{step.order}）", comment.strip())
    _mail(r.requester.email, f"差し戻し: {r.title}", "returned.txt", {"r": r, "line": line})
    return r


def _lock_own(user, request_id: int) -> Request:
    r = Request.objects.select_for_update().get(pk=request_id)
    if r.requester_id != user.pk:
        raise WorkflowError("自分の依頼ではありません。")
    return r


@transaction.atomic
def pull_back(user, request_id: int) -> Request:
    """引き戻し（回覧中だけ）→ 引き戻し中。承認待ちの人の URL は使えなくなる"""
    r = _lock_own(user, request_id)
    if r.status != S.CIRCULATING:
        raise WorkflowError(f"「{r.get_status_display()}」の依頼は引き戻せません。")
    r.steps.filter(round=r.round, decision=D.WAITING).update(decision=D.CANCELLED)
    r.status = S.PULLED_BACK
    r.save(update_fields=["status", "updated_at"])
    _log(r, user, "引き戻した")
    return r


@transaction.atomic
def resubmit(user, request_id: int, *, title: str, detail: str, due_date, approver_ids: list[int], drawing=None) -> Request:
    """直して出し直す → 回覧中。最初の回覧者からやり直す"""
    r = _lock_own(user, request_id)
    if r.status not in (S.RETURNED, S.PULLED_BACK):
        raise WorkflowError(f"「{r.get_status_display()}」の依頼は出し直せません。")
    approvers = _check_approvers(user, approver_ids)
    r.title, r.detail, r.due_date, r.drawing = title, detail, due_date, drawing
    r.round += 1
    r.status = S.CIRCULATING
    r.save()
    _start_round(r, approvers)
    _log(r, user, f"出し直した（{r.round} 回目）")
    return r


@transaction.atomic
def withdraw(user, request_id: int) -> Request:
    """取り下げ（差し戻し中・引き戻し中だけ）"""
    r = _lock_own(user, request_id)
    if r.status not in (S.RETURNED, S.PULLED_BACK):
        raise WorkflowError(f"「{r.get_status_display()}」の依頼は取り下げられません。")
    r.status = S.WITHDRAWN
    r.save(update_fields=["status", "updated_at"])
    _log(r, user, "取り下げた")
    return r


@transaction.atomic
def assign(worker, request_id: int) -> Request:
    """担当を決める（回覧完了 → 対応中）"""
    if worker.role not in (User.Role.WORKER, User.Role.ADMIN):
        raise WorkflowError("担当者だけができます。")
    r = Request.objects.select_for_update().get(pk=request_id)
    if r.status != S.APPROVED:
        raise WorkflowError(f"「{r.get_status_display()}」の依頼は、担当を決められません。")
    r.assignee, r.status = worker, S.IN_PROGRESS
    r.save(update_fields=["assignee", "status", "updated_at"])
    _log(r, worker, "担当になった")
    return r


@transaction.atomic
def complete(worker, request_id: int) -> Request:
    """引き渡した（対応中 → 完了）"""
    r = Request.objects.select_for_update().get(pk=request_id)
    if r.status != S.IN_PROGRESS or r.assignee_id != worker.pk:
        raise WorkflowError("担当している、対応中の依頼だけを完了にできます。")
    r.status = S.DONE
    r.save(update_fields=["status", "updated_at"])
    _log(r, worker, "完了にした")
    _mail(r.requester.email, f"完了: {r.title}", "done.txt", {"r": r})
    return r


@transaction.atomic
def resend(admin, step_id: int) -> Step:
    """承認のお願いを送り直す（前の URL は使えなくなる）。毎朝の通知でも使う"""
    step = Step.objects.select_for_update().select_related("request", "approver").get(pk=step_id)
    if step.decision != D.WAITING or step.request.status != S.CIRCULATING:
        raise WorkflowError("承認を待っている回覧だけを送り直せます。")
    _send_approval_mail(step)
    _log(step.request, admin, f"承認のお願いを送り直した（回覧者{step.order}）")
    return step


def notify_due_soon(r: Request) -> None:
    """納期が近いことを、依頼者と担当者に知らせる（毎朝の通知で使う）"""
    for to in {r.requester.email, *([r.assignee.email] if r.assignee else [])}:
        _mail(to, f"納期が近づいています: {r.title}", "due_soon.txt", {"r": r})

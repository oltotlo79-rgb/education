import re
from datetime import date, timedelta
from io import StringIO
from pathlib import Path
import tempfile

from django.core import mail
from django.core.files.uploadedfile import SimpleUploadedFile
from django.core.management import call_command
from django.core.management.base import CommandError
from django.test import TestCase, override_settings
from django.utils import timezone
from rest_framework.test import APIClient

from accounts.models import User
from workflow import services
from workflow.models import Request, Step

URL_RE = re.compile(r"/approve/([A-Za-z0-9_-]+)/")


def token_in(message) -> str:
    """メールの本文から、承認の URL の文字列を取り出す"""
    return URL_RE.search(message.body).group(1)


@override_settings(EMAIL_BACKEND="django.core.mail.backends.locmem.EmailBackend", MAIL_REDIRECT_TO="")
class WorkflowTests(TestCase):
    def setUp(self):
        self.requester = User.objects.create_user("000001", "pass-for-test-1", name="依頼 一郎", email="req@example.com")
        self.a1 = User.objects.create_user("000002", "pass-for-test-2", name="承認 一子", email="a1@example.com")
        self.a2 = User.objects.create_user("000003", "pass-for-test-3", name="承認 二郎", email="a2@example.com")
        self.worker = User.objects.create_user("000004", "pass-for-test-4", name="担当 三郎", email="w@example.com",
                                               role=User.Role.WORKER)

    def submit(self, approvers=None):
        with self.captureOnCommitCallbacks(execute=True):  # 保存が確定した後のメールも、テストの中で送る
            return services.submit(self.requester, title="治具の製作", detail="検査用", due_date=date(2026, 11, 30),
                                   approver_ids=[a.pk for a in (approvers or [self.a1, self.a2])])

    def act(self, func, *args):
        with self.captureOnCommitCallbacks(execute=True):
            return func(*args)

    def test_出すと1人目にメールが届き回覧中になる(self):
        r = self.submit()
        self.assertEqual(r.status, Request.Status.CIRCULATING)
        self.assertEqual(len(mail.outbox), 1)
        self.assertEqual(mail.outbox[0].to, ["a1@example.com"])
        self.assertIn("承認のお願い", mail.outbox[0].subject)

    def test_全員が承認すると回覧完了になり担当者に知らせる(self):
        self.submit()
        self.act(services.approve, token_in(mail.outbox[0]))
        self.assertEqual(mail.outbox[1].to, ["a2@example.com"])          # 2 人目へ
        r = self.act(services.approve, token_in(mail.outbox[1]))
        self.assertEqual(r.status, Request.Status.APPROVED)
        self.assertEqual({m.to[0] for m in mail.outbox[2:]}, {"req@example.com", "w@example.com"})

    def test_URLは1回しか使えない(self):
        self.submit()
        token = token_in(mail.outbox[0])
        self.act(services.approve, token)
        with self.assertRaisesMessage(services.WorkflowError, "承認済みです"):
            services.approve(token)

    def test_期限が切れたURLは使えない(self):
        self.submit()
        Step.objects.update(token_expires_at=timezone.now() - timedelta(seconds=1))
        with self.assertRaisesMessage(services.WorkflowError, "期限が切れています"):
            services.approve(token_in(mail.outbox[0]))

    def test_差し戻すとコメントが追記され依頼者にメールが届く(self):
        self.submit()
        r = self.act(services.return_request, token_in(mail.outbox[0]), "寸法を追記してください")
        self.assertEqual(r.status, Request.Status.RETURNED)
        self.assertRegex(r.return_comments, r"^【\d\d/\d\d \d\d:\d\d 回覧者1 承認 一子】寸法を追記してください$")
        self.assertEqual(mail.outbox[-1].to, ["req@example.com"])

    def test_理由の無い差し戻しは断る(self):
        self.submit()
        with self.assertRaisesMessage(services.WorkflowError, "理由を書いてください"):
            services.return_request(token_in(mail.outbox[0]), "  ")

    def test_引き戻すと承認待ちの人のURLが使えなくなる(self):
        r = self.submit()
        token = token_in(mail.outbox[0])
        services.pull_back(self.requester, r.pk)
        check = services.check_token(token)
        self.assertIsNone(check.step)
        self.assertEqual(check.message, "依頼者により引き戻されました。")

    def test_出し直すと最初の回覧者からやり直す(self):
        r = self.submit()
        self.act(services.approve, token_in(mail.outbox[0]))            # 1 人目は承認済み
        self.act(services.return_request, token_in(mail.outbox[1]), "図面を直して")
        r = self.act(lambda: services.resubmit(self.requester, r.pk, title="治具の製作（直し）", detail="",
                                               due_date=date(2026, 12, 5), approver_ids=[self.a1.pk, self.a2.pk]))
        self.assertEqual((r.status, r.round), (Request.Status.CIRCULATING, 2))
        self.assertEqual(mail.outbox[-1].to, ["a1@example.com"])

    def test_回覧中はほかの人の依頼を引き戻せない(self):
        r = self.submit()
        with self.assertRaisesMessage(services.WorkflowError, "自分の依頼ではありません"):
            services.pull_back(self.a1, r.pk)

    def test_回覧者の決まり(self):
        with self.assertRaisesMessage(services.WorkflowError, "同じ人を 2 回は選べません"):
            self.submit([self.a1, self.a1])
        with self.assertRaisesMessage(services.WorkflowError, "自分自身は回覧者にできません"):
            self.submit([self.requester])

    @override_settings(MAIL_REDIRECT_TO="it@example.com")
    def test_検証ではメールの宛先を書きかえる(self):
        self.submit()
        self.assertEqual(mail.outbox[0].to, ["it@example.com"])
        self.assertTrue(mail.outbox[0].subject.startswith("【検証】"))
        self.assertIn("本来の宛先: a1@example.com", mail.outbox[0].body)


@override_settings(EMAIL_BACKEND="django.core.mail.backends.locmem.EmailBackend", MAIL_REDIRECT_TO="")
class ApiTests(TestCase):
    def setUp(self):
        self.yamada = User.objects.create_user("012345", "pass-for-test-1", name="山田 太郎", email="y@example.com")
        self.sato = User.objects.create_user("012346", "pass-for-test-2", name="佐藤 花子", email="s@example.com")
        self.suzuki = User.objects.create_user("012347", "pass-for-test-3", name="鈴木 一郎", email="z@example.com")
        self.client = APIClient()  # テストの道具は、CSRF の札の確かめを省く（CSRF は後編で、画面から確かめる）

    def login(self, user, password):
        res = self.client.post("/api/login/", {"employee_no": user.employee_no, "password": password}, format="json")
        self.assertEqual(res.status_code, 200)

    def test_ログインしないと403(self):
        self.assertEqual(self.client.get("/api/requests/").status_code, 403)

    def test_出すと番号が返りほかの人からは見えない(self):
        self.login(self.yamada, "pass-for-test-1")
        res = self.client.post("/api/requests/", {"title": "治具の製作", "due_date": "2026-11-30",
                                                  "approver_ids": [self.sato.pk]}, format="json")
        self.assertEqual(res.status_code, 201)
        pk = res.json()["id"]
        self.client.post("/api/logout/")
        self.login(self.suzuki, "pass-for-test-3")
        self.assertEqual(self.client.get(f"/api/requests/{pk}/").status_code, 404)

    def test_件名に改行は入れられない(self):
        # 画面の入力欄は 1 行だが、細工した送信なら改行を入れられる。メールの件名に入ると送れなくなる（10 の巻）
        self.login(self.yamada, "pass-for-test-1")
        res = self.client.post("/api/requests/", {"title": "治具の製作\r\nBcc: evil@example.com", "due_date": "2026-11-30",
                                                  "approver_ids": [self.sato.pk]}, format="json")
        self.assertEqual(res.status_code, 400)
        self.assertEqual(res.json()["title"], ["件名に、改行などは入れられません。"])

    def test_図面の形が違えば断る(self):
        self.login(self.yamada, "pass-for-test-1")
        res = self.client.post("/api/requests/", {"title": "a", "due_date": "2026-11-30", "approver_ids": [self.sato.pk],
                                                  "drawing": {"version": 2}}, format="json")
        self.assertEqual(res.status_code, 400)
        self.assertIn("drawing", res.json())

    def test_拡張子と中身が違うファイルは断る(self):
        self.login(self.yamada, "pass-for-test-1")
        pk = self.client.post("/api/requests/", {"title": "a", "due_date": "2026-11-30", "approver_ids": [self.sato.pk]},
                              format="json").json()["id"]
        with tempfile.TemporaryDirectory() as media, override_settings(MEDIA_ROOT=media):
            fake = SimpleUploadedFile("drawing.pdf", b"MZ\x90\x00 this is not a pdf")  # 名前は PDF、中身は違う
            res = self.client.post(f"/api/requests/{pk}/attachments/", {"file": fake}, format="multipart")
            self.assertEqual(res.status_code, 400)
            self.assertEqual(res.json()["detail"], "ファイルの中身が、拡張子と合いません。")
            png = SimpleUploadedFile("photo.png", b"\x89PNG\r\n\x1a\n" + b"0" * 100)
            res = self.client.post(f"/api/requests/{pk}/attachments/", {"file": png}, format="multipart")
            self.assertEqual(res.status_code, 201)


class ImportEmployeesTests(TestCase):
    def write_csv(self, text: str) -> Path:
        f = tempfile.NamedTemporaryFile("w", suffix=".csv", delete=False, encoding="utf-8-sig", newline="")
        f.write(text)
        f.close()
        return Path(f.name)

    def test_正しいCSVは取り込む(self):
        p = self.write_csv("従業員番号,氏名,所属,役職,メールアドレス\r\n012345,山田 太郎,設計部,課長,taro@example.com\r\n")
        out = StringIO()
        call_command("import_employees", str(p), stdout=out)
        self.assertIn("新しく 1 件", out.getvalue())
        self.assertEqual(User.objects.get(employee_no="012345").name, "山田 太郎")

    def test_1行でも間違いがあれば1件も取り込まない(self):
        p = self.write_csv("従業員番号,氏名,所属,役職,メールアドレス\r\n012345,山田 太郎,設計部,課長,taro@example.com\r\n"
                           "12346,佐藤 花子,設計部,係長,hanako@example.com\r\n")
        with self.assertRaises(CommandError):
            call_command("import_employees", str(p), stdout=StringIO(), stderr=StringIO())
        self.assertFalse(User.objects.filter(employee_no="012345").exists())

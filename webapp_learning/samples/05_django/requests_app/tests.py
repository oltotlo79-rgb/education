from datetime import date

from django.contrib.auth import get_user_model
from rest_framework.test import APITestCase

from .models import Request


class RequestApiTests(APITestCase):
    def setUp(self):
        User = get_user_model()
        self.yamada = User.objects.create_user("012345", password="pass-for-test-1", first_name="太郎", last_name="山田")
        self.sato = User.objects.create_user("012346", password="pass-for-test-2")
        Request.objects.create(title="治具の製作", due_date=date(2026, 11, 10), requester=self.yamada)

    def test_ログインしていないと見られない(self):
        res = self.client.get("/api/requests/")
        self.assertEqual(res.status_code, 403)

    def test_自分の依頼だけが見える(self):
        self.client.force_authenticate(self.sato)
        res = self.client.get("/api/requests/")
        self.assertEqual(res.status_code, 200)
        self.assertEqual(res.json(), [])

    def test_依頼を作ると回覧中になり依頼者は自分になる(self):
        self.client.force_authenticate(self.yamada)
        res = self.client.post("/api/requests/", {"title": "棚の取り付け", "due_date": "2026-11-20", "status": "done"})
        self.assertEqual(res.status_code, 201)
        self.assertEqual(res.json()["status"], "circulating")  # 送った "done" は無視される
        self.assertEqual(res.json()["requester_name"], "太郎 山田")

    def test_件名が空なら断る(self):
        self.client.force_authenticate(self.yamada)
        res = self.client.post("/api/requests/", {"title": "  ", "due_date": "2026-11-20"})
        self.assertEqual(res.status_code, 400)
        self.assertIn("title", res.json())

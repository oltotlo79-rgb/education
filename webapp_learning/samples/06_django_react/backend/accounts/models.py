from django.contrib.auth.models import AbstractBaseUser, BaseUserManager, PermissionsMixin
from django.db import models


class UserManager(BaseUserManager):
    def create_user(self, employee_no: str, password: str | None = None, **extra):
        user = self.model(employee_no=employee_no, **extra)
        if password:
            user.set_password(password)  # パスワードは、そのままではなく、元に戻せない形にして保存する
        else:
            user.set_unusable_password()
        user.save(using=self._db)
        return user

    def create_superuser(self, employee_no: str, password: str, **extra):
        extra.update(is_staff=True, is_superuser=True, role=User.Role.ADMIN)
        return self.create_user(employee_no, password, **extra)


class User(AbstractBaseUser, PermissionsMixin):
    """社員。従業員番号でログインする"""

    class Role(models.TextChoices):
        MEMBER = "member", "一般"
        WORKER = "worker", "担当者"   # 依頼に対応する部署の人
        ADMIN = "admin", "管理者"

    employee_no = models.CharField("従業員番号", max_length=6, unique=True)
    name = models.CharField("氏名", max_length=50)  # 「山田 太郎」の順でそのまま持つ
    dept = models.CharField("所属", max_length=50, blank=True)
    job_title = models.CharField("役職", max_length=50, blank=True)
    email = models.EmailField("メールアドレス")
    role = models.CharField("権限", max_length=10, choices=Role.choices, default=Role.MEMBER)
    is_active = models.BooleanField("在籍", default=True)
    is_staff = models.BooleanField("管理画面に入れる", default=False)

    objects = UserManager()

    USERNAME_FIELD = "employee_no"
    EMAIL_FIELD = "email"
    REQUIRED_FIELDS = ["name", "email"]

    class Meta:
        verbose_name = "社員"
        verbose_name_plural = "社員"
        ordering = ["employee_no"]

    def __str__(self) -> str:
        return f"{self.name}（{self.employee_no}）"

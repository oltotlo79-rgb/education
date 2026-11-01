from django.contrib import admin

from .models import User


@admin.register(User)
class UserAdmin(admin.ModelAdmin):
    list_display = ["employee_no", "name", "dept", "job_title", "role", "is_active"]
    list_filter = ["role", "is_active", "dept"]
    search_fields = ["employee_no", "name", "dept"]
    fields = ["employee_no", "name", "dept", "job_title", "email", "role", "is_active", "is_staff"]

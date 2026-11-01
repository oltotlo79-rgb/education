from django.contrib import admin
from django.urls import path

from accounts import views as accounts_views
from workflow import api, approve_views

urlpatterns = [
    path("admin/", admin.site.urls),
    # メールの URL から開く承認の画面（ログインは要らない）
    path("approve/<str:token>/", approve_views.approve_page, name="approve"),
    # React の画面が呼ぶ API
    path("api/login/", accounts_views.login_view),
    path("api/logout/", accounts_views.logout_view),
    path("api/me/", api.me),
    path("api/requests/", api.requests_view),
    path("api/requests/<int:pk>/", api.request_detail),
    path("api/requests/<int:pk>/attachments/", api.upload_attachment),
    path("api/requests/<int:pk>/<str:action>/", api.request_action),
    path("api/attachments/<int:attachment_id>/", api.download_attachment),
    path("api/employees/", api.employees),
    path("api/stats/", api.stats),
]

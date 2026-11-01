from django.contrib import admin
from django.urls import path, re_path

from accounts import views as accounts_views
from workflow import api, approve_views, pages

urlpatterns = [
    path("admin/", admin.site.urls),
    path("health", pages.health),
    # メールの URL から開く承認の画面（ログインは要らない）
    path("approve/<str:token>/", approve_views.approve_page, name="approve"),
    # React の画面が呼ぶ API
    path("api/csrf/", pages.csrf),
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
    # それ以外の URL は、React の画面（index.html）を返す。React が画面を切りかえる（後編）
    re_path(r"^(?!api/|admin/|static/|approve/).*$", pages.IndexView.as_view()),
]

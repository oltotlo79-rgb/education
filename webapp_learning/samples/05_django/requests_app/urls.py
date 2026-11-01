from django.urls import include, path
from rest_framework.routers import DefaultRouter

from . import views

router = DefaultRouter()
router.register("requests", views.RequestViewSet, basename="request")

urlpatterns = [
    path("", views.request_list, name="request_list"),
    path("api/", include(router.urls)),
]

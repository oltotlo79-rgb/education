"""外から来た物（図面の JSON・添付ファイル）を確かめる。画面の確かめとは別に、必ずサーバーでも確かめる"""

import json

from django.core.exceptions import ValidationError

MAX_DRAWING_BYTES = 5 * 1024 * 1024
MAX_FILE_BYTES = 10 * 1024 * 1024
SHAPE_KINDS = {"line", "rect", "text"}

# ファイルの先頭の数バイトで、本当の種類を見分ける（名前の拡張子は、いくらでもごまかせる）
SIGNATURES = {
    ".jpg": [b"\xff\xd8\xff"],
    ".jpeg": [b"\xff\xd8\xff"],
    ".png": [b"\x89PNG\r\n\x1a\n"],
    ".pdf": [b"%PDF-"],
}


def validate_drawing(value) -> None:
    if value is None:
        return
    if len(json.dumps(value, ensure_ascii=False).encode()) > MAX_DRAWING_BYTES:
        raise ValidationError("図面は 5MB までです。")
    if not isinstance(value, dict) or value.get("version") != 1 or not isinstance(value.get("shapes"), list):
        raise ValidationError("図面の形が違います。")
    for shape in value["shapes"]:
        if not isinstance(shape, dict) or shape.get("kind") not in SHAPE_KINDS:
            raise ValidationError("図面に、知らない図形が入っています。")


def validate_upload(name: str, head: bytes, size: int) -> None:
    ext = "." + name.rsplit(".", 1)[-1].lower() if "." in name else ""
    if ext not in SIGNATURES:
        raise ValidationError("添付できるのは JPG・PNG・PDF だけです。")
    if size > MAX_FILE_BYTES:
        raise ValidationError("添付ファイルは 1 つ 10MB までです。")
    if not any(head.startswith(sig) for sig in SIGNATURES[ext]):
        raise ValidationError("ファイルの中身が、拡張子と合いません。")

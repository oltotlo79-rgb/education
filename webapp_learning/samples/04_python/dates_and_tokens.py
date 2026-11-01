"""日時（日本時間）と、承認の URL に使う、推測できない文字列"""

import hashlib
import hmac
import secrets
from datetime import datetime, timedelta
from zoneinfo import ZoneInfo

JST = ZoneInfo("Asia/Tokyo")  # Windows では tzdata の部品が要る（requirements.txt）

# 1. 日時: 時間帯付きで扱う
issued = datetime(2026, 11, 1, 9, 30, tzinfo=JST)
expires = issued + timedelta(days=14)            # 14 日で期限切れ
print("発行:", issued.isoformat())
print("期限:", expires.strftime("%Y-%m-%d %H:%M"))
print("期限切れか（11/20 の時点）:", datetime(2026, 11, 20, tzinfo=JST) > expires)


def next_run(now: datetime, hour: int = 8) -> datetime:
    """次の「毎朝 8:00」を返す"""
    today = now.replace(hour=hour, minute=0, second=0, microsecond=0)
    return today if now < today else today + timedelta(days=1)


print("次の 8:00（7:59 から）:", next_run(datetime(2026, 11, 2, 7, 59, tzinfo=JST)).isoformat())
print("次の 8:00（8:01 から）:", next_run(datetime(2026, 11, 2, 8, 1, tzinfo=JST)).isoformat())

# 2. 承認の URL に入れる、推測できない文字列（毎回違う）
token = secrets.token_urlsafe(32)
print("文字列の長さ:", len(token))
print("2 回作ると違うか:", token != secrets.token_urlsafe(32))

# 3. データベースには、文字列そのものではなく「指紋（ハッシュ値）」を保存する
stored = hashlib.sha256(token.encode()).hexdigest()
print("保存する指紋:", stored[:16] + "…")


def matches(presented: str, stored_hash: str) -> bool:
    """URL で届いた文字列が、保存した指紋と合うか（比べる時間で中身を推測されない比べ方）"""
    return hmac.compare_digest(hashlib.sha256(presented.encode()).hexdigest(), stored_hash)


print("本物の URL:", matches(token, stored))
print("にせの URL:", matches("aaaa", stored))

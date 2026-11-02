# 付録 B-15: 日付と時刻でよく使う書き方
from datetime import date, datetime, timedelta
from zoneinfo import ZoneInfo

TOKYO = ZoneInfo("Asia/Tokyo")   # 日本時間（Windows では tzdata の部品が要る）

# 日付だけ: date。日付と時刻: datetime（tzinfo で「どこの時間か」を付ける）
due = date(2026, 11, 10)
sent = datetime(2026, 11, 2, 8, 0, tzinfo=TOKYO)
print(due, sent)

# 形を決めて文字にする（strftime）・文字から読む（strptime）
print(due.strftime("%Y年%m月%d日"), sent.strftime("%H:%M"))
print(datetime.strptime("2026/11/02 08:00", "%Y/%m/%d %H:%M"))
print(date.fromisoformat("2026-12-01"))       # 2026-12-01 の形はこれで読める

# 足し算・引き算: timedelta（日・時間・分の長さ）
print(due + timedelta(days=7))                # 1 週間後
print(sent + timedelta(minutes=60))           # 60 分後（ログアウトの時刻など）
left = due - date(2026, 11, 2)
print(left.days, "日")

# 比べる・曜日（月曜が 0）
print(due > date(2026, 11, 1), "月火水木金土日"[due.weekday()])

# 世界標準時（UTC）と日本時間の変換。保存は UTC、表示は日本時間、がよくある決まり
utc = sent.astimezone(ZoneInfo("UTC"))
print(utc.isoformat(), "→", utc.astimezone(TOKYO).isoformat())

# 「今」は datetime.now(TOKYO)。動かすたびに変わるので、ここでは形だけ確かめる
now = datetime.now(TOKYO)
print(now.tzinfo, now.year >= 2026)

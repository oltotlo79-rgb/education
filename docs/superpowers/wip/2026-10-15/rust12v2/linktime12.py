"""Windows の実行ファイルの、リンクした時刻（UNIX 秒）を、UTC の日時でも表示する。"""
import datetime
import struct
import sys

data = open(sys.argv[1], "rb").read()
pe = struct.unpack_from("<I", data, 0x3C)[0]
t = struct.unpack_from("<I", data, pe + 8)[0]
print(f"リンクした時刻: {t}（{datetime.datetime.fromtimestamp(t, datetime.timezone.utc):%Y-%m-%d %H:%M:%S} UTC）")

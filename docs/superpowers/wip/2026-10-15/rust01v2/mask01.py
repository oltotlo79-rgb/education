"""第1巻の出力のユーザー名を伏せる（標準入力 → 標準出力）。

- C:\\Users\\<名前>\\r01\\ （スクラッチへの短いつなぎ）と C:\\Users\\<名前>\\ を、C:\\Users\\...\\ にする
- {:?} の形（\\ が2つ）と、/ の形も同じように伏せる
- WSL の /home/<名前> は /home/... にする
"""
import re
import sys

BS = chr(92)
NAME = "olt" + "ot"
data = sys.stdin.buffer.read().decode("utf-8", "replace")
# スクラッチの rust01 の長いパス（表示の途中で折り返されていることもある）
parts = ["C:", "Users", NAME, "AppData", "Local", "Temp", "claude",
         "C--Users-" + NAME + "-Documents-git-projects-def",
         "f344debc-e3c6-4c0d-93d7-e55dbf9acedd", "scratchpad", "rust01"]
pattern = (r"\s*" + re.escape(BS)).join(re.escape(p) for p in parts) + r"\s*" + re.escape(BS)
data = re.sub(pattern, lambda m: BS.join(["C:", "Users", "..."]) + BS, data)
for sep in (BS, BS * 2, "/"):
    home = sep.join(["C:", "Users", NAME])
    data = data.replace(home + sep + "r01" + sep, sep.join(["C:", "Users", "..."]) + sep)
    data = data.replace(home + sep, sep.join(["C:", "Users", "..."]) + sep)
    data = data.replace(home, sep.join(["C:", "Users", "..."]))
data = data.replace("/c/Users/" + NAME + "/r01/", "/c/Users/.../")
data = data.replace("/c/Users/" + NAME + "/", "/c/Users/.../")
data = data.replace("/home/yuya", "/home/...")
data = data.replace(BS + "home" + BS + "yuya", BS + "home" + BS + "...")
data = data.replace("/mnt/c/Users/" + NAME + "/", "/mnt/c/Users/.../")
data = data.replace("\r", "")
# 色を付ける印（ESC [ … m）を消す
data = re.sub(chr(27) + r"\[[0-9;]*m", "", data)
sys.stdout.write(data)

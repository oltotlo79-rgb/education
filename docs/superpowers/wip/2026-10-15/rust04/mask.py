import re
import sys

BS = chr(92)
data = sys.stdin.buffer.read().decode("utf-8", "replace")
stop = "[^" + re.escape(BS) + r"\s)'\"/]+"
# C:\Users\...\name -> ...\name   (keep only the last path segment)
win = r"[A-Za-z]:" + re.escape(BS) + "Users" + re.escape(BS) + r"[^\s)'\"]*" + re.escape(BS) + "(" + stop + ")"
data = re.sub(win, lambda m: "..." + BS + m.group(1), data)
data = re.sub(r"[A-Za-z]:/Users/[^\s)'\"]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = re.sub(r"/c/Users/[^\s)'\"]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = "".join(l for l in data.splitlines(True) if "Blocking waiting for file lock" not in l)
sys.stdout.write(data)

import re
import sys

BS = chr(92)
E = re.escape(BS)
data = sys.stdin.buffer.read().decode("utf-8", "replace")
stop = "[^" + E + r"\s)'\"/]+"
# cargo registry: C:\Users\name\.cargo\registry\src\index.crates.io-xxxx\crate-1.0\src\a.rs -> ...\crate-1.0\src\a.rs
reg = r"[A-Za-z]:" + E + "Users" + E + r"[^" + E + r"\s]+" + E + r"\.cargo" + E + "registry" + E + "src" + E + r"[^" + E + r"\s]+" + E
data = re.sub(reg, lambda m: "..." + BS, data)
# C:\Users\...\name -> ...\name   (keep only the last path segment)
win = r"[A-Za-z]:" + E + "Users" + E + r"[^\s)'\"`]*" + E + "(" + stop + ")"
data = re.sub(win, lambda m: "..." + BS + m.group(1), data)
data = re.sub(r"[A-Za-z]:/Users/[^\s)'\"`]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = re.sub(r"/c/Users/[^\s)'\"`]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = "".join(l for l in data.splitlines(True) if "Blocking waiting for file lock" not in l)
sys.stdout.write(data)

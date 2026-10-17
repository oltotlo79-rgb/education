"""Mask personal paths in captured output (stdin -> stdout).

- <scratch>/rust10/target-*      -> target
- <scratch>/rust10/target        -> target
- <scratch>/rust10/<project>     -> ...\\<project>
- other C:\\Users\\... paths      -> ...\\<last segment>
"""
import re
import sys

BS = "\\"
data = sys.stdin.buffer.read().decode("utf-8", "replace")

scratch_variants = [
    r"C:\Users\oltot\AppData\Local\Temp\claude\C--Users-oltot-Documents-git-projects-def\f344debc-e3c6-4c0d-93d7-e55dbf9acedd\scratchpad",
    r"C:/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad",
    r"/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad",
]
for base in scratch_variants:
    for sep in ("\\", "/"):
        prefix = base + sep + "rust10" + sep
        # target directories -> target
        data = re.sub(re.escape(prefix) + r"target(?:-[A-Za-z0-9_]+)?", "target", data)
        data = data.replace(prefix, "..." + BS)
# mixed separators like target\debug -> keep
data = re.sub(r"target/debug\\", r"target\\debug\\", data)
data = re.sub(r"target/release\\", r"target\\release\\", data)

E = re.escape(BS)
stop = "[^" + E + r"\s)'\"/]+"
win = r"[A-Za-z]:" + E + "Users" + E + r"[^\s)'\"`]*" + E + "(" + stop + ")"
data = re.sub(win, lambda m: "..." + BS + m.group(1), data)
data = re.sub(r"[A-Za-z]:/Users/[^\s)'\"`]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = "".join(l for l in data.splitlines(True) if "Blocking waiting for file lock" not in l)
sys.stdout.write(data)

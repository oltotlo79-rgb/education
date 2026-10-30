"""出力のパスを伏せる（標準入力 → 標準出力）。

- スクラッチの target-shared        -> target
- スクラッチの rustNN/<プロジェクト> -> ...<区切り><プロジェクト>
- そのほかのユーザーフォルダの下のパス -> ...<区切り><最後の部分>
"""
import re
import sys

BS = chr(92)
SCRATCH = BS.join(
    [
        "C:",
        "Users",
        "oltot",
        "AppData",
        "Local",
        "Temp",
        "claude",
        "C--Users-oltot-Documents-git-projects-def",
        "f344debc-e3c6-4c0d-93d7-e55dbf9acedd",
        "scratchpad",
    ]
)
data = sys.stdin.buffer.read().decode("utf-8", "replace")
bases = [
    SCRATCH,
    SCRATCH.replace(BS, "/"),
    "/c/" + SCRATCH[3:].replace(BS, "/"),
]
for base in bases:
    for sep in (BS, "/"):
        data = data.replace(base + sep + "target-shared", "target")
        data = data.replace(base + sep + "target11", "target")
        pattern = re.escape(base + sep) + r"rust\d\d\w*" + re.escape(sep)
        data = re.sub(pattern, lambda m: "..." + BS, data)
# {:?} で表示したパス（\ が \\ になっている）も同じように伏せる
DBS = BS * 2
data = data.replace(SCRATCH.replace(BS, DBS) + DBS + "target-shared", "target")
data = data.replace(SCRATCH.replace(BS, DBS) + DBS + "target11", "target")
data = re.sub(re.escape(SCRATCH.replace(BS, DBS) + DBS) + r"rust\d\d\w*" + re.escape(DBS), lambda m: "..." + DBS, data)
# target/debug\ のように区切りが混ざったものを、\ にそろえる
for profile in ("debug", "release"):
    data = data.replace("target/" + profile + BS, "target" + BS + profile + BS)
E = re.escape(BS)
stop = "[^" + E + r"\s)'\"/]+"
win = r"[A-Za-z]:" + E + "Users" + E + r"[^\s)'\"`]*" + E + "(" + stop + ")"
data = re.sub(win, lambda m: "..." + BS + m.group(1), data)
data = re.sub(r"[A-Za-z]:/Users/[^\s)'\"`]*/(" + stop + ")", lambda m: ".../" + m.group(1), data)
data = "".join(line for line in data.splitlines(True) if "Blocking waiting for file lock" not in line)
sys.stdout.write(data)

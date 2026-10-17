#!/bin/bash
S=/c/Users/oltot/AppData/Local/Temp/claude/C--Users-oltot-Documents-git-projects-def/f344debc-e3c6-4c0d-93d7-e55dbf9acedd/scratchpad/rust00
cd /c/Users/oltot/Documents/git-projects/def
OUT="Rust/00_学習ガイド_コンピュータとWebから完成像まで.md"
python - "$S/parts" "$OUT" <<'PY'
import sys, pathlib
parts = pathlib.Path(sys.argv[1])
names = ["00_front.md"] + [f"unit{i:02d}.md" for i in range(1, 11)] + ["99_end.md"]
out = ""
for n in names:
    t = (parts / n).read_text(encoding="utf-8").replace("\r\n", "\n").rstrip("\n") + "\n\n"
    out += t
out = out.rstrip("\n") + "\n"
pathlib.Path(sys.argv[2]).write_bytes(out.encode("utf-8"))
print("lines", out.count("\n"))
PY
PYTHONIOENCODING=utf-8 python $S/../ref/check_md.py "$OUT"

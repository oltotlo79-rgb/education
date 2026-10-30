"""第9巻の出力のパスを伏せる。maskx.py のあとに、作業用のフォルダ名（book・u05work など）を消す。"""
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BS = chr(92)
data = sys.stdin.buffer.read()
out = subprocess.run(
    [sys.executable, str(HERE.parent / "maskx.py")], input=data, capture_output=True
).stdout.decode("utf-8")
out = out.replace("\r\n", "\n")
out = re.sub(r"\.\.\.\\(?:book|u\d\dwork)\\", lambda m: "..." + BS, out)
out = re.sub(r"\.\.\./(?:book|u\d\dwork)/", ".../", out)
sys.stdout.buffer.write(out.encode("utf-8"))

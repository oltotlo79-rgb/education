"""mask09.py の後に、book フォルダの名前も消す（...\\book\\rustnote -> ...\\rustnote）。"""
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BS = chr(92)
data = sys.stdin.buffer.read()
out = subprocess.run(
    [sys.executable, str(HERE / "mask09.py")], input=data, capture_output=True
).stdout.decode("utf-8")
out = out.replace("\r\n", "\n")
out = out.replace("..." + BS + "book" + BS, "..." + BS).replace(".../book/", ".../")
sys.stdout.buffer.write(out.encode("utf-8"))

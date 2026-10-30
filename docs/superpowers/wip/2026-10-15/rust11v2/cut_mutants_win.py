"""cargo install cargo-mutants の失敗のログから、本に載せる部分を取り出し、パスを伏せる。"""
import re
from pathlib import Path

here = Path(__file__).resolve().parent
lines = (here / "mutants_install.log").read_text(encoding="utf-8", errors="replace").replace("\r", "").split("\n")
first = next(i for i, line in enumerate(lines) if line.startswith("error[E0425]"))
block = lines[first : first + 13]
tail = [line for line in lines if line.startswith("error: could not compile") or line.startswith("error: failed to compile")]
count = sum(1 for line in lines if line.startswith("error[E0425]"))
head = [line for line in lines if "Compiling winapi v0.2" in line or "Installing cargo-mutants" in line][:2]
out = head + ["..."] + block + ["...", f"（error[E0425] が全部で {count} 個）"] + tail
text = "\n".join(out)
text = re.sub(r"C:\\Users\\[^\\]+\\\.cargo\\registry\\src\\[^\\]+\\", lambda m: "...\\registry\\", text)
text = re.sub(r"C:\\Users\\[^\\]+\\AppData\\Local\\Temp\\cargo-install[A-Za-z0-9]+", lambda m: "...\\cargo-install...", text)
(here / "out" / "u06_mutants_windows.txt").write_text(text + "\n", encoding="utf-8", newline="\n")
print(text)

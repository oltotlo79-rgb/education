import subprocess, sys
from pathlib import Path
R = Path(__file__).resolve().parent
parts = ["front"] + [f"unit{n:02d}" for n in range(1, 11)] + ["back"]
chunks = []
for name in parts:
    src = R / "parts" / f"{name}.md"
    dst = R / "out" / f"{name}.md"
    subprocess.run([sys.executable, str(R / "fill.py"), str(src), str(dst)], check=True, capture_output=True)
    chunks.append(dst.read_text(encoding="utf-8").strip("\n") + "\n")
out = Path(r"C:/Users/oltot/Documents/git-projects/def/Rust/02_Rust基礎_値_型_制御_データ構造.md")
out.write_text("\n".join(chunks), encoding="utf-8", newline="\n")
print("written", out, sum(c.count("\n") for c in chunks) + len(chunks) - 1, "lines")

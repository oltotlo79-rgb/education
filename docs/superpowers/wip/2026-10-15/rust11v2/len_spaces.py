import re
from pathlib import Path

out = Path(__file__).resolve().parent / "out"
for i in (1, 2, 3):
    s = (out / f"u04_spaces{i}.txt").read_text(encoding="utf-8")
    raw = re.search(r'minimal failing input: input = "(.*)"', s).group(1)
    text = re.sub(r"\\u\{([0-9a-f]+)\}", lambda m: chr(int(m.group(1), 16)), raw)
    text = text.replace(r"\t", "\t")
    print(i, "全体", len(text), "前後の空白を除くと", len(text.strip()), "successes", re.search(r"successes: (\d+)", s).group(1))
